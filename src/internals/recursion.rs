use super::cookies_io::CookiesIOExt;
use crate::{base::client::Client, internals::fields::DEFAULT_HEADERS};
use async_recursion::async_recursion;
use reqwest::{Response, StatusCode, Url};

#[async_recursion]
pub async fn recursion_cookies_handle(
    client: impl Client + Clone + Send + 'async_recursion,
    url: &str,
    cookie_store_url: &Url,
) -> Result<Response, String> {
    if let Ok(response) = client
        .reqwest_client()
        .lock()
        .await
        .get(url)
        .headers(DEFAULT_HEADERS.clone())
        .send()
        .await
    {
        client
            .cookies()
            .lock()
            .unwrap()
            .copy_cookies_raw(&Url::parse(url).unwrap(), cookie_store_url);
        if response.status() == StatusCode::FOUND {
            return recursion_cookies_handle(
                client,
                response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap(),
                cookie_store_url,
            )
            .await;
        }
        return Ok(response);
    }

    Err(format!("Can't get `{}`", url))
}
