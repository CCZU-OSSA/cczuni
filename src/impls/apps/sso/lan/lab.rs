use std::{collections::HashMap, io::ErrorKind};

use crate::{
    base::{app::Application, client::Client, typing::EmptyOrErr},
    internals::{
        fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN},
        recursion::recursion_redirect_handle,
    },
};

pub struct LabApplication<C> {
    client: C,
}

impl<C: Client> Application<C> for LabApplication<C> {
    async fn from_client(client: C) -> Self {
        Self { client }
    }
}

impl<C: Client + Clone + Send + Sync> LabApplication<C> {
    pub async fn exam_login(&self) -> EmptyOrErr {
        let api = format!(
            "{}?service=https://sysaqgl.cczu.edu.cn/labexam/examIDSLogin.php",
            ROOT_SSO_LOGIN
        );
        recursion_redirect_handle(self.client.clone(), &api)
            .await
            .map_err(|error| tokio::io::Error::new(ErrorKind::Other, error))?;

        Ok(())
    }

    pub async fn exam_increase_thirty_secs(&self) -> EmptyOrErr {
        let api = "https://sysaqgl.cczu.edu.cn/labexam/exam_xuexi_online.php";
        let mut params = HashMap::new();
        params.insert("cmd", "xuexi_online");
        self.client
            .reqwest_client()
            .post(api)
            .headers(DEFAULT_HEADERS.clone())
            .form(&params)
            .send()
            .await
            .map_err(|error| tokio::io::Error::new(ErrorKind::Other, error.to_string()))?;
        Ok(())
    }
}
