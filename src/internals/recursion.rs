use std::io::ErrorKind;

use crate::{
    base::{client::Client, typing::TorErr},
    internals::fields::DEFAULT_HEADERS,
};
use async_recursion::async_recursion;
use reqwest::{header::LOCATION, Response, StatusCode};

#[async_recursion]
pub async fn recursion_redirect_handle(
    client: impl Client + Clone + Send + 'async_recursion,
    url: &str,
) -> TorErr<Response> {
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
                response.headers().get(LOCATION).unwrap().to_str().unwrap(),
            )
            .await;
        }
        return Ok(response);
    }

    Err(tokio::io::Error::new(
        ErrorKind::Other,
        format!("Can't get `{}`", url),
    ))
}
