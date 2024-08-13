use crate::{base::client::Client, internals::fields::DEFAULT_HEADERS};
use async_recursion::async_recursion;
use reqwest::{Response, StatusCode};

#[async_recursion]
pub async fn recursion_redirect_handle(
    client: impl Client + Clone + Send + 'async_recursion,
    url: &str,
) -> Result<Response, String> {
    if let Ok(response) = client
        .reqwest_client()
        .get(url)
        .headers(DEFAULT_HEADERS.clone())
        .send()
        .await
    {
        if response.status() == StatusCode::FOUND {
            return recursion_redirect_handle(
                client,
                response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
            .await;
        }
        return Ok(response);
    }

    Err(format!("Can't get `{}`", url))
}
