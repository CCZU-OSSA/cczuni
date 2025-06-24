use crate::internals::fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN, WECHAT_APP_API};
use const_format::formatcp;
use reqwest::{Method, StatusCode, Url};
use std::{collections::HashMap, time::Duration};
use tokio::task::JoinSet;

pub async fn url_status_code(url: Url) -> StatusCode {
    reqwest::Client::new()
        .request(Method::HEAD, url)
        .send()
        .await
        .map(|e| e.status())
        .unwrap_or(StatusCode::BAD_REQUEST)
}

pub async fn services_status_code() -> HashMap<&'static str, StatusCode> {
    let mut status_map = HashMap::new();
    let client = reqwest::Client::new();
    // Example URLs for different services
    let sites = [
        ("SSO", ROOT_SSO_LOGIN),
        ("WeChat", formatcp!("{}/api/login", WECHAT_APP_API)),
        ("WebVPN", "https://zmvpn.cczu.edu.cn/enlink/sso/login"),
    ];

    let mut tasks = JoinSet::new();

    for (name, url) in sites.into_iter() {
        let client = client.clone();
        tasks.spawn(async move {
            (
                name,
                client
                    .request(Method::OPTIONS, url)
                    .headers(DEFAULT_HEADERS.clone())
                    .timeout(Duration::from_secs(3))
                    //  .headers(DEFAULT_HEADERS.clone())
                    .send()
                    .await
                    .map(|response| response.status())
                    .unwrap_or(StatusCode::REQUEST_TIMEOUT),
            )
        });
    }

    tasks
        .join_all()
        .await
        .into_iter()
        .for_each(|(name, status)| {
            status_map.insert(name, status);
        });

    status_map
}

#[tokio::test]
async fn test() {
    println!("{:?}", services_status_code().await)
}
