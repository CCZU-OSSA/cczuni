use std::{collections::HashMap, future::Future, sync::LazyLock};

use reqwest::Url;

use crate::impls::login::sso_status::SSOLoginStatus;
use crate::{base::client::Client, impls::login::sso_type::SSOLoginConnectType};

use super::{
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

pub trait Redirect {
    fn redirect(&self, url: impl Into<String>) -> impl Future<Output = String>;
    fn initialize_url(&self, url: impl Into<Url>) -> impl Future<Output = ()>;
}

impl<C: Client> Redirect for C {
    async fn redirect(&self, url: impl Into<String>) -> String {
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

    async fn initialize_url(&self, url: impl Into<Url>) {
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
            .copy_cookies(from, &url.into());
    }
}
