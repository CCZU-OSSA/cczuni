use std::{collections::HashMap, future::Future, io::ErrorKind};

use crate::{
    base::{
        client::Client,
        typing::{convert_error, TorErr},
    },
    internals::{
        cookies_io::CookiesIOExt,
        fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN, ROOT_VPN_URL},
        recursion::recursion_redirect_handle,
    },
};
use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{cookie::Cookie, header::LOCATION, Response, StatusCode};
use scraper::{Html, Selector};

use super::sso_type::{ElinkLoginInfo, SSOLoginConnectType, SSOUniversalLoginInfo};

pub trait SSOUniversalLogin {
    /// This method implements [`ROOT_SSO`] url login.
    ///
    /// You can only get the ElinkLoginInfo in WebVPN Mode...
    fn sso_universal_login(&self) -> impl Future<Output = TorErr<Option<ElinkLoginInfo>>>;

    fn sso_service_login(
        &self,
        service: impl Into<String>,
    ) -> impl Future<Output = TorErr<Response>>;
}

impl<C: Client + Clone + Send> SSOUniversalLogin for C {
    async fn sso_universal_login(&self) -> TorErr<Option<ElinkLoginInfo>> {
        let login = universal_sso_login(self.clone()).await?;
        self.properties().write().await.insert(
            SSOLoginConnectType::key(),
            login.login_connect_type.clone().into(),
        );

        match login.login_connect_type {
            SSOLoginConnectType::WEBVPN => {
                let response = login.response;

                if let Some(cookie) = &response
                    .cookies()
                    .filter(|cookie| cookie.name() == "clientInfo")
                    .collect::<Vec<Cookie>>()
                    .first()
                {
                    let json =
                        String::from_utf8(BASE64_STANDARD.decode(cookie.value()).unwrap()).unwrap();
                    let data: ElinkLoginInfo = serde_json::from_str(&json)?;

                    Ok(Some(data))
                } else {
                    Err(tokio::io::Error::new(
                        ErrorKind::Other,
                        "Get `EnlinkLoginInfo` failed",
                    ))
                }
            }
            SSOLoginConnectType::COMMON => Ok(None),
        }
    }

    async fn sso_service_login(&self, service: impl Into<String>) -> TorErr<Response> {
        service_sso_login(self.clone(), service).await
    }
}

async fn universal_sso_login(client: impl Client + Clone + Send) -> TorErr<SSOUniversalLoginInfo> {
    let response = client
        .reqwest_client()
        .get(ROOT_SSO_LOGIN)
        .send()
        .await
        .map_err(convert_error)?;
    let status = response.status();
    // use webvpn
    if status == StatusCode::FOUND {
        // redirect to webvpn root
        // recursion to get the login page
        let response = recursion_redirect_handle(
            client.clone(),
            response
                .headers()
                .get("location")
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .await
        .map_err(convert_error)?;

        let url = response.url().clone();
        let dom = response.text().await.unwrap();
        let mut form = parse_hidden_values(dom.as_str());

        let account = client.account();
        form.insert("username".into(), account.user);
        form.insert("password".into(), BASE64_STANDARD.encode(account.password));

        let response = client
            .reqwest_client()
            .post(url)
            .form(&form)
            .send()
            .await
            .map_err(convert_error)?;

        let redirect_location_header = response.headers().get("location");
        if let None = redirect_location_header {
            return Err(tokio::io::Error::new(
                ErrorKind::NotFound,
                "Redirect to None",
            ));
        }
        let redirect_location = redirect_location_header.unwrap().to_str().unwrap();

        let response = client
            .reqwest_client()
            .get(redirect_location)
            .headers(DEFAULT_HEADERS.clone())
            .send()
            .await
            .map_err(convert_error)?;

        client
            .cookies()
            .lock()
            .unwrap()
            .add_reqwest_cookies(response.cookies(), &ROOT_VPN_URL);
        Ok(SSOUniversalLoginInfo {
            response,
            login_connect_type: SSOLoginConnectType::WEBVPN,
        })
    }
    // connect `cczu` and don't need to redirect
    else if status == StatusCode::OK {
        Ok(SSOUniversalLoginInfo {
            response: service_sso_login(client, "").await?,
            login_connect_type: SSOLoginConnectType::COMMON,
        })
    } else {
        Err(tokio::io::Error::new(ErrorKind::Other, "Login Failed"))
    }
}

async fn service_sso_login(
    client: impl Client + Clone + Send,
    service: impl Into<String>,
) -> TorErr<Response> {
    let api = format!("{}?service={}", ROOT_SSO_LOGIN, service.into());
    let response = client
        .reqwest_client()
        .get(api.clone())
        .send()
        .await
        .map_err(convert_error)?;

    // Has Logined before
    if response.status() == StatusCode::FOUND {
        return Ok(recursion_redirect_handle(
            client,
            response
                .headers()
                .get(LOCATION)
                .ok_or(tokio::io::Error::new(
                    ErrorKind::Other,
                    "Get Location Failed",
                ))?
                .to_str()
                .map_err(convert_error)?,
        )
        .await?);
    }

    let dom = response.text().await.unwrap();
    let mut login_param = parse_hidden_values(dom.as_str());
    let account = client.account();
    login_param.insert("username".into(), account.user);
    login_param.insert("password".into(), BASE64_STANDARD.encode(account.password));

    let response = client
        .reqwest_client()
        .post(api)
        .form(&login_param)
        .headers(DEFAULT_HEADERS.clone())
        .send()
        .await
        .map_err(convert_error)?;

    if response.status() == StatusCode::FOUND {
        Ok(recursion_redirect_handle(
            client,
            response
                .headers()
                .get(LOCATION)
                .ok_or(tokio::io::Error::new(
                    ErrorKind::Other,
                    "Get Location Failed",
                ))?
                .to_str()
                .map_err(convert_error)?,
        )
        .await?)
    } else {
        Ok(response)
    }
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
