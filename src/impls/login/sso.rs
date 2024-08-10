use std::{collections::HashMap, future::Future};

use crate::{
    base::{client::Client, typing::EmptyOrErr},
    internals::{
        cookies_io::CookiesIOExt,
        fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN, ROOT_VPN_URL},
        recursion::recursion_cookies_handle,
    },
};
use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::StatusCode;
use scraper::{Html, Selector};

use super::sso_type::{LoginConnectType, UniversalSSOLogin};

pub trait SSOLogin {
    fn sso_login(&self) -> impl Future<Output = EmptyOrErr>;
}

impl<C: Client + Clone + Send> SSOLogin for C {
    async fn sso_login(&self) -> EmptyOrErr {
        let login_info = universal_sso_login(self.clone()).await?;
        self.properties().write().await.insert(
            LoginConnectType::key(),
            login_info.login_connect_type.into(),
        );
        Ok(())
    }
}

pub async fn universal_sso_login(
    client: impl Client + Clone + Send,
) -> Result<UniversalSSOLogin, &'static str> {
    if let Ok(response) = client.reqwest_client().get(ROOT_SSO_LOGIN).send().await {
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
                    .post(url)
                    .form(&login_param)
                    .send()
                    .await
                {
                    let redirect_location_header = response.headers().get("location");
                    if let None = redirect_location_header {
                        return Err("跳转 WebVPN 失败");
                    }
                    let redirect_location = redirect_location_header.unwrap().to_str().unwrap();
                    if let Ok(response) = client
                        .reqwest_client()
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

pub fn parse_hidden_values(html: &str) -> HashMap<String, String> {
    let mut hidden_values = HashMap::new();
    let dom = Html::parse_document(html);
    let input_hidden_selector = Selector::parse(r#"input[type="hidden"]"#).unwrap();
    let tags_hidden = dom.select(&input_hidden_selector);

    tags_hidden.for_each(|tag_hidden| {
        hidden_values.insert(
            tag_hidden.attr("name").unwrap().to_string(),
            tag_hidden.attr("value").unwrap().to_string(),
        );
    });

    hidden_values
}
