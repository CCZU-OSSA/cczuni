use crate::{base::client::Client, internals::fields::DEFAULT_HEADERS};
use anyhow::{Context, Result};
use async_recursion::async_recursion;
use reqwest::{Response, StatusCode, header::LOCATION};

#[async_recursion]
pub async fn recursion_redirect_handle(
    client: impl Client + Clone + Send + 'async_recursion,
    url: &str,
) -> Result<Response> {
    let response = client
        .reqwest_client()
        .get(url)
        .headers(DEFAULT_HEADERS.clone())
        .send()
        .await
        .context(format!("Failed to get '{}'", url))?;

    if response.status() == StatusCode::FOUND {
        let location = response
            .headers()
            .get(LOCATION)
            .context("No location header in redirect response")?
            .to_str()
            .context("Invalid location header")?;
        return recursion_redirect_handle(client, location).await;
    }

    Ok(response)
}
