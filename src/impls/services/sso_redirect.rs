use std::{collections::HashMap, future::Future, sync::LazyLock};

use reqwest::Url;

use crate::base::client::Client;
use crate::impls::login::{sso_status::SSOLoginStatus, sso_type::SSOLoginConnectType};
use crate::internals::{
    cookies_io::CookiesIOExt,
    fields::{ROOT_SSO_URL, ROOT_VPN_URL},
};

pub const STATIC_SERVER_MAP: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // jwcas
    map.insert(
        "http://219.230.159.132".into(),
        "https://zmvpn.cczu.edu.cn/http/webvpndc2d086cb5b297c15e661687e73c1549".into(),
    );

    map
});

pub trait SSORedirect {
    fn sso_redirect(&self, url: impl Into<String>) -> impl Future<Output = String>;
    fn sso_initialize_url(&self, url: impl Into<String>) -> impl Future<Output = ()>;
}

impl<C: Client> SSORedirect for C {
    async fn sso_redirect(&self, url: impl Into<String>) -> String {
        let url = url.into();

        match self
            .sso_login_connect_type()
            .await
            .expect("Can't get login connect type! Need Login?")
        {
            SSOLoginConnectType::WEBVPN => STATIC_SERVER_MAP.get(&url).unwrap_or(&url).to_string(),
            SSOLoginConnectType::COMMON => url,
        }
    }

    async fn sso_initialize_url(&self, url: impl Into<String>) {
        let url: String = url.into();
        let from = match self
            .sso_login_connect_type()
            .await
            .expect("Can't get login connect type! Need Login?")
        {
            SSOLoginConnectType::WEBVPN => &ROOT_VPN_URL,
            SSOLoginConnectType::COMMON => &ROOT_SSO_URL,
        };
        self.cookies()
            .lock()
            .unwrap()
            .copy_cookies(from, &Url::parse(url.as_str()).unwrap());
    }
}
