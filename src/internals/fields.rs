use std::{collections::HashMap, sync::LazyLock};

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
#[allow(dead_code)]
pub const ROOT_SSO: &'static str = "http://sso.cczu.edu.cn";
#[allow(dead_code)]
pub const ROOT_SSO_URL: LazyLock<Url> = LazyLock::new(|| Url::parse(ROOT_SSO).unwrap());
pub const ROOT_SSO_LOGIN: &'static str = "http://sso.cczu.edu.cn/sso/login";
// pub const ROOT_SSO_LOGIN_URL: Lazy<Url> = Lazy::new(|| Url::parse(ROOT_SSO_LOGIN).unwrap());

pub const ROOT_VPN: &'static str = "https://zmvpn.cczu.edu.cn";
pub const ROOT_VPN_URL: LazyLock<Url> = LazyLock::new(|| Url::parse(ROOT_VPN).unwrap());
#[allow(dead_code)]
pub const ROOT_YWTB: &'static str = "http://ywtb.cczu.edu.cn";
#[allow(dead_code)]
pub const WEBVPN_SERVER_MAP: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // jwcas
    map.insert(
        "http://219.230.159.132".into(),
        "https://zmvpn.cczu.edu.cn/http/webvpndc2d086cb5b297c15e661687e73c1549".into(),
    );

    map
});
#[allow(dead_code)]
pub const WECHAT_APP_API: &'static str = "http://202.195.102.7:8180";
