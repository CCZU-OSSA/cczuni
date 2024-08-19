use std::{collections::HashMap, future::Future, io::ErrorKind};

use crate::{
    base::{client::Client, typing::TorErr},
    internals::{
        cookies_io::CookiesIOExt,
        fields::{DEFAULT_HEADERS, ROOT_SSO, ROOT_SSO_LOGIN, ROOT_VPN_URL, ROOT_YWTB},
        recursion::recursion_redirect_handle,
    },
};
use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{cookie::Cookie, StatusCode, Url};
use scraper::{Html, Selector};

use super::sso_type::{ElinkLoginInfo, SSOLoginConnectType, SSOUniversalLoginInfo};

pub trait SSOUniversalLogin {
    /// This method implements [`ROOT_SSO`] url login.
    ///
    /// You can only get the ElinkLoginInfo in WebVPN Mode...
    fn sso_universal_login(&self) -> impl Future<Output = TorErr<Option<ElinkLoginInfo>>>;
}

impl<C: Client + Clone + Send> SSOUniversalLogin for C {
    async fn sso_universal_login(&self) -> TorErr<Option<ElinkLoginInfo>> {
        let login_info = universal_sso_login(self.clone()).await?;
        self.properties().write().await.insert(
            SSOLoginConnectType::key(),
            login_info.login_connect_type.clone().into(),
        );

        match login_info.login_connect_type {
            SSOLoginConnectType::WEBVPN => {
                let response = login_info.response;

                if let Some(cookie) = &response
                    .cookies()
                    .filter(|cookie| cookie.name() == "clientInfo")
                    .collect::<Vec<Cookie>>()
                    .first()
                {
                    let json =
                        String::from_utf8(BASE64_STANDARD.decode(cookie.value()).unwrap()).unwrap();
                    let data: ElinkLoginInfo = serde_json::from_str(&json).unwrap();

                    Ok(Some(data))
                } else {
                    Err(tokio::io::Error::new(
                        ErrorKind::Other,
                        "Get `EnlinkLoginInfo` failed",
                    ))
                }
            }
            SSOLoginConnectType::COMMON => {
                self.cookies().lock().unwrap().copy_cookies(
                    &ROOT_SSO.parse::<Url>().unwrap(),
                    &format!("{}/pc/index.html", ROOT_YWTB)
                        .parse::<Url>()
                        .unwrap(),
                );

                Ok(None)
            }
        }
    }
}

async fn universal_sso_login(client: impl Client + Clone + Send) -> TorErr<SSOUniversalLoginInfo> {
    if let Ok(response) = client.reqwest_client().get(ROOT_SSO_LOGIN).send().await {
        // use webvpn
        if response.status() == StatusCode::FOUND {
            // redirect to webvpn root
            // recursion to get the login page

            if let Ok(response) = recursion_redirect_handle(
                client.clone(),
                response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap(),
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
                        return Err(tokio::io::Error::new(
                            ErrorKind::NotFound,
                            "Redirect to None",
                        ));
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
                        return Ok(SSOUniversalLoginInfo {
                            response,
                            login_connect_type: SSOLoginConnectType::WEBVPN,
                        });
                    };
                };
            }
        }
        // connect `cczu` and don't need to redirect
        // TODO: Remove the cookies Copy here may cause some problems, need debug
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
                return Ok(SSOUniversalLoginInfo {
                    response,
                    login_connect_type: SSOLoginConnectType::COMMON,
                });
            };
        }
    }
    Err(tokio::io::Error::new(ErrorKind::Other, "Login Failed"))
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
