use std::{collections::HashMap, future::Future};

use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::StatusCode;

use crate::{
    base::{client::Client, typing::DetailErrResult},
    impls::client::DefaultClient,
    internals::{
        cookies_io::CookiesIOExt,
        fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN, ROOT_VPN_URL},
        recursion::recursion_cookies_handle,
    },
};

use super::sso_type::{LoginConnectType, UniversalSSOLogin};

pub trait SSOLogin {
    fn sso_login(&self) -> impl Future<Output = DetailErrResult>;
}

impl SSOLogin for DefaultClient {
    async fn sso_login(&self) -> DetailErrResult {
        universal_sso_login(self.clone()).await?;
        Ok(())
    }
}

pub async fn universal_sso_login(
    client: impl Client + Clone + Send,
) -> Result<UniversalSSOLogin, &'static str> {
    if let Ok(response) = client
        .reqwest_client()
        .lock()
        .await
        .get(ROOT_SSO_LOGIN)
        .send()
        .await
    {
        // use webvpn
        if response.status() == StatusCode::FOUND {
            // redirect to webvpn root
            // recursion to get the login page

            if let Ok(response) = recursion_cookies_handle(
                client.clone(),
                response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap(),
                &ROOT_VPN_URL,
            )
            .await
            {
                let url = response.url().clone();
                let dom = response.text().await.unwrap();
                let mut login_param = parse_hidden_values(dom.as_str());

                let account = client.account();
                login_param.insert("username".into(), account.user);
                login_param.insert("password".into(), BASE64_STANDARD.encode(account.password));

                if let Ok(response) = client
                    .reqwest_client()
                    .lock()
                    .await
                    .post(url)
                    .form(&login_param)
                    .send()
                    .await
                {
                    let redirect_location_header = response.headers().get("location");
                    if let None = redirect_location_header {
                        return Err("跳转WebVPN失败");
                    }
                    let redirect_location = redirect_location_header.unwrap().to_str().unwrap();
                    if let Ok(response) = client
                        .reqwest_client()
                        .lock()
                        .await
                        .get(redirect_location)
                        .headers(DEFAULT_HEADERS.clone())
                        .send()
                        .await
                    {
                        client
                            .cookies()
                            .lock()
                            .unwrap()
                            .add_reqwest_cookies(response.cookies(), &ROOT_VPN_URL);
                        return Ok(UniversalSSOLogin {
                            response,
                            login_connect_type: LoginConnectType::WEBVPN,
                        });
                    };
                };
            }
        }
        // connect `cczu` and don't need to redirect
        if response.status() == StatusCode::OK {
            let dom = response.text().await.unwrap();
            let mut login_param = parse_hidden_values(dom.as_str());
            let account = client.account();
            login_param.insert("username".into(), account.user);
            login_param.insert("password".into(), BASE64_STANDARD.encode(account.password));

            if let Ok(response) = client
                .reqwest_client()
                .lock()
                .await
                .post(ROOT_SSO_LOGIN)
                .form(&login_param)
                .send()
                .await
            {
                return Ok(UniversalSSOLogin {
                    response,
                    login_connect_type: LoginConnectType::COMMON,
                });
            };
        }
    }
    Err("无法登录！请检查账户密码！")
}
fn parse_hidden_values(_html: &str) -> HashMap<String, String> {
    todo!("May be use a async parser here");
}
