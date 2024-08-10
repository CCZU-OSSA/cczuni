use std::future::Future;

use reqwest::StatusCode;

use crate::{base::client::Client, internals::fields::ROOT_SSO_LOGIN};

use super::sso_type::SSOLoginConnectType;

pub trait SSOLoginStatus {
    fn sso_login_available(&self) -> impl Future<Output = bool>;
    fn sso_login_connect_type(&self) -> impl Future<Output = Option<SSOLoginConnectType>>;
}

impl<C: Client> SSOLoginStatus for C {
    async fn sso_login_available(&self) -> bool {
        if let Ok(response) = self
            .reqwest_client()
            .get(format!(
                "{}?service=http://ywtb.cczu.edu.cn/pc/index.html",
                ROOT_SSO_LOGIN
            ))
            .send()
            .await
        {
            if response.status() == StatusCode::OK {
                return false;
            }

            if response.status() == StatusCode::FOUND {
                let location = response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap();
                if location.contains("sso/login") {
                    return false;
                }
                return true;
            }
        }
        false
    }

    async fn sso_login_connect_type(&self) -> Option<SSOLoginConnectType> {
        if let Some(property) = self
            .properties()
            .read()
            .await
            .get(SSOLoginConnectType::key())
        {
            Some(property.clone().into())
        } else {
            None
        }
    }
}
