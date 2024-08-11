use std::sync::LazyLock;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Url,
};

pub static DEFAULT_HEADERS: LazyLock<HeaderMap> = LazyLock::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/116.0",
        ),
    );
    headers
});
pub const ROOT_SSO: &'static str = "http://sso.cczu.edu.cn";
pub const ROOT_SSO_URL: LazyLock<Url> = LazyLock::new(|| Url::parse(ROOT_SSO).unwrap());
pub const ROOT_SSO_LOGIN: &'static str = "http://sso.cczu.edu.cn/sso/login";

pub const ROOT_VPN: &'static str = "https://zmvpn.cczu.edu.cn";
pub const ROOT_VPN_URL: LazyLock<Url> = LazyLock::new(|| Url::parse(ROOT_VPN).unwrap());
pub const ROOT_YWTB: &'static str = "http://ywtb.cczu.edu.cn";
#[allow(unused)]
pub const WECHAT_APP_API: &'static str = "http://202.195.102.7:8180";
