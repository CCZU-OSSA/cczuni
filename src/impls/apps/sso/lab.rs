use std::{collections::HashMap, io::ErrorKind};

use crate::{
    base::{app::Application, client::Client, typing::EmptyOrErr},
    impls::login::{
        sso::SSOUniversalLogin, sso_status::SSOLoginStatus, sso_type::SSOLoginConnectType,
    },
    internals::{
        fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN},
        recursion::recursion_redirect_handle,
    },
};

static LABAPP_ROOT: &'static str = "https://sysaqgl.cczu.edu.cn";

pub struct LabApplication<C> {
    client: C,
}

impl<C: Client> Application<C> for LabApplication<C> {
    async fn from_client(client: C) -> Self {
        Self { client }
    }
}

impl<C: Client + Clone + Send + Sync> LabApplication<C> {
    /// Support LAN/WLAN
    pub async fn exam_login(&self) -> EmptyOrErr {
        let api = format!(
            "{}?service={LABAPP_ROOT}/labexam/examIDSLogin.php",
            ROOT_SSO_LOGIN
        );

        let need_login: bool;

        let connect_type = self.client.sso_login_connect_type().await;

        if let None = connect_type {
            need_login = true;
        } else if let Some(SSOLoginConnectType::WEBVPN) = connect_type {
            need_login = true;
        } else if let SSOLoginConnectType::COMMON = self.client.sso_login_type().await? {
            need_login = true;
        } else {
            need_login = false;
        }

        if need_login {
            self.client
                .sso_service_login(format!("{LABAPP_ROOT}/labexam/examIDSLogin.php"))
                .await?;
        }

        recursion_redirect_handle(self.client.clone(), &api)
            .await
            .map_err(|error| tokio::io::Error::new(ErrorKind::Other, error))?;

        Ok(())
    }

    pub async fn exam_increase_thirty_secs(&self) -> EmptyOrErr {
        let api = format!("{LABAPP_ROOT}/labexam/exam_xuexi_online.php");
        let mut params = HashMap::new();
        params.insert("cmd", "xuexi_online");
        let resp = self
            .client
            .reqwest_client()
            .post(api)
            .headers(DEFAULT_HEADERS.clone())
            .form(&params)
            .send()
            .await
            .map_err(|error| tokio::io::Error::new(ErrorKind::Other, error.to_string()))?;
        println!("{}", resp.text().await.unwrap());
        Ok(())
    }
}
