use std::{collections::HashMap, future::Future};

use super::sso_type::{ElinkLoginInfo, SSOLoginConnectType, SSOUniversalLoginInfo};
use crate::{
    base::client::Client,
    internals::{
        cookies_io::CookiesIOExt,
        fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN, ROOT_VPN_URL},
        recursion::recursion_redirect_handle,
    },
};
use anyhow::{bail, Context, Result};
use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{cookie::Cookie, header::LOCATION, Response, StatusCode};
use scraper::{Html, Selector};

pub trait SSOUniversalLogin {
    /// This method implements [`ROOT_SSO`] url login.
    ///
    /// You can only get the ElinkLoginInfo in WebVPN Mode...
    fn sso_universal_login(&self) -> impl Future<Output = Result<Option<ElinkLoginInfo>>>;

    fn sso_service_login(
        &self,
        service: impl Into<String>,
    ) -> impl Future<Output = Result<Response>>;
}

impl<C: Client + Clone + Send> SSOUniversalLogin for C {
    async fn sso_universal_login(&self) -> Result<Option<ElinkLoginInfo>> {
        let login = universal_sso_login(self.clone()).await?;
        self.properties().write().await.insert(
            SSOLoginConnectType::key(),
            login.login_connect_type.clone().into(),
        );

        match login.login_connect_type {
            SSOLoginConnectType::WEBVPN => Ok(Some(serde_json::from_str(&String::from_utf8(
                BASE64_STANDARD.decode(
                    login
                        .response
                        .cookies()
                        .filter(|cookie| cookie.name() == "clientInfo")
                        .collect::<Vec<Cookie>>()
                        .first()
                        .context("Get `EnlinkLoginInfo` Failed")?
                        .value(),
                )?,
            )?)?)),
            SSOLoginConnectType::COMMON => Ok(None),
        }
    }

    async fn sso_service_login(&self, service: impl Into<String>) -> Result<Response> {
        service_sso_login(self.clone(), service).await
    }
}

async fn universal_sso_login(client: impl Client + Clone + Send) -> Result<SSOUniversalLoginInfo> {
    let response = client.reqwest_client().get(ROOT_SSO_LOGIN).send().await?;
    let status = response.status();
    // use webvpn
    if status == StatusCode::FOUND {
        // redirect to webvpn root
        // recursion to get the login page
        let location = response
            .headers()
            .get("location")
            .context("No location header in initial response")?
            .to_str()
            .context("Invalid location header")?;
        let response = recursion_redirect_handle(client.clone(), location).await?;

        let url = response.url().clone();
        let dom = response.text().await?;
        let mut form = parse_hidden_values(dom.as_str())?;

        let account = client.account();
        form.insert("username".into(), account.user);
        form.insert("password".into(), BASE64_STANDARD.encode(account.password));

        let response = client.reqwest_client().post(url).form(&form).send().await?;

        let redirect_location = response
            .headers()
            .get("location")
            .context("No redirect location after login")?
            .to_str()
            .context("Invalid redirect location")?;

        let response = client
            .reqwest_client()
            .get(redirect_location)
            .headers(DEFAULT_HEADERS.clone())
            .send()
            .await?;

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
        bail!("Login Failed with status: {}", status)
    }
}

async fn service_sso_login(
    client: impl Client + Clone + Send,
    service: impl Into<String>,
) -> Result<Response> {
    let api = format!("{}?service={}", ROOT_SSO_LOGIN, service.into());
    let response = client.reqwest_client().get(api.clone()).send().await?;

    // Has Logined before
    if response.status() == StatusCode::FOUND {
        let location = response
            .headers()
            .get(LOCATION)
            .context("Get Location Failed")?
            .to_str()
            .context("Invalid location header")?;
        return Ok(recursion_redirect_handle(client, location).await?);
    }

    let dom = response.text().await?;
    let mut login_param = parse_hidden_values(dom.as_str())?;
    let account = client.account();
    login_param.insert("username".into(), account.user);
    login_param.insert("password".into(), BASE64_STANDARD.encode(account.password));

    let response = client
        .reqwest_client()
        .post(api)
        .form(&login_param)
        .headers(DEFAULT_HEADERS.clone())
        .send()
        .await?;

    if response.status() == StatusCode::FOUND {
        let location = response
            .headers()
            .get(LOCATION)
            .context("Get Location Failed")?
            .to_str()
            .context("Invalid location header")?;
        Ok(recursion_redirect_handle(client, location).await?)
    } else {
        Ok(response)
    }
}

pub fn parse_hidden_values(html: &str) -> Result<HashMap<String, String>> {
    let mut hidden_values = HashMap::new();
    let dom = Html::parse_document(html);
    let input_hidden_selector = Selector::parse(r#"input[type="hidden"]"#).unwrap();
    let tags_hidden = dom.select(&input_hidden_selector);

    for tag_hidden in tags_hidden {
        let name = tag_hidden
            .attr("name")
            .context("Hidden input missing name attribute")?;
        let value = tag_hidden
            .attr("value")
            .context("Hidden input missing value attribute")?;
        hidden_values.insert(name.to_string(), value.to_string());
    }

    Ok(hidden_values)
}
