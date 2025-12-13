use std::future::Future;

use reqwest::{StatusCode, header::LOCATION};

use super::sso_type::SSOLoginConnectType;
use crate::{base::client::Client, internals::fields::ROOT_SSO_LOGIN};
use anyhow::{Result, bail};

pub trait SSOLoginStatus {
    fn sso_login_available(&self) -> impl Future<Output = bool>;
    fn sso_login_connect_type(&self) -> impl Future<Output = Option<SSOLoginConnectType>>;
    fn sso_login_type(&self) -> impl Future<Output = Result<SSOLoginConnectType>>;
    fn sso_login_type_write(&self) -> impl Future<Output = Result<SSOLoginConnectType>>;
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
                let location = response.headers().get(LOCATION).unwrap().to_str().unwrap();
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

    async fn sso_login_type(&self) -> Result<SSOLoginConnectType> {
        if let Some(connect_type) = self.sso_login_connect_type().await {
            return Ok(connect_type);
        }

        match self
            .reqwest_client()
            .get(ROOT_SSO_LOGIN)
            .send()
            .await?
            .status()
        {
            StatusCode::OK => Ok(SSOLoginConnectType::COMMON),
            StatusCode::FOUND => Ok(SSOLoginConnectType::WEBVPN),
            _ => bail!("Status Code Error"),
        }
    }

    async fn sso_login_type_write(&self) -> Result<SSOLoginConnectType> {
        let connect = self.sso_login_type().await?;
        let locker = self.properties();
        let mut guard = locker.write().await;
        if guard.contains_key(SSOLoginConnectType::key()) {
            panic!("Can't write to existing property")
        }

        guard.insert(SSOLoginConnectType::key(), connect.clone().into());

        Ok(connect)
    }
}
