use std::collections::HashMap;

use crate::{
    base::client::Client,
    impls::login::sso_type::ElinkLoginInfo,
    internals::fields::{DEFAULT_HEADERS, ROOT_VPN},
};
use aes::{
    Aes128Enc,
    cipher::{BlockEncryptMut, KeyIvInit, block_padding::Pkcs7},
};
use anyhow::{Context, Result, bail};
use base64::{Engine, prelude::BASE64_STANDARD};
use cbc::Encryptor;
use rand::Rng;
use reqwest::{StatusCode, cookie::Cookie};

pub type CbcAES128Enc = Encryptor<Aes128Enc>;

pub trait WebVPNLogin {
    fn webvpn_login(&self) -> impl std::future::Future<Output = Result<ElinkLoginInfo>>;
}

impl<C: Client> WebVPNLogin for C {
    async fn webvpn_login(&self) -> Result<ElinkLoginInfo> {
        let account = self.account();
        let url = format!("{}/enlink/sso/login/submit", ROOT_VPN);
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut rng = rand::rng();
        let mut token = (0..16)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as u8
            })
            .collect::<Vec<u8>>();
        let iv = token.clone();
        token.reverse();
        let key = token.clone();
        let encryptor = CbcAES128Enc::new(key.as_slice().into(), iv.as_slice().into());
        let pwd_clone = account.password;
        let raw_pwd = pwd_clone.as_bytes();
        let pwd_len = raw_pwd.len();
        let mut buf = [0u8; 256];
        buf[..pwd_len].copy_from_slice(&raw_pwd);
        let encrypt_buf = encryptor
            .encrypt_padded_mut::<Pkcs7>(&mut buf, pwd_len)
            .map_err(|_| anyhow::anyhow!("Password encryption failed"))?;
        let encrypt_pwd = BASE64_STANDARD.encode(encrypt_buf);
        let mut data: HashMap<&'static str, String> = HashMap::new();
        data.insert("username", account.user);
        data.insert("password", encrypt_pwd);
        data.insert(
            "token",
            token.iter().map(|char| char.clone() as char).collect(),
        );
        data.insert("language", "zh-CN,zh;q=0.9,en;q=0.8".into());

        let response = self
            .reqwest_client()
            .post(url)
            .header("Refer", format!("{}/enlink/sso/login", ROOT_VPN))
            .header("Origin", ROOT_VPN)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .headers(DEFAULT_HEADERS.clone())
            .form(&data)
            .send()
            .await
            .context("Failed to send login request")?;

        if response.status() == StatusCode::FOUND {
            let cookies: Vec<Cookie> = response
                .cookies()
                .filter(|cookie| cookie.name() == "clientInfo")
                .collect();
            let cookie = cookies.first().context("No clientInfo cookie found")?;

            let decoded = BASE64_STANDARD
                .decode(cookie.value())
                .context("Failed to decode clientInfo cookie")?;
            let json = String::from_utf8(decoded).context("Invalid UTF-8 in decoded cookie")?;
            let elink_info: ElinkLoginInfo =
                serde_json::from_str(&json).context("Failed to parse ElinkLoginInfo from JSON")?;
            Ok(elink_info)
        } else {
            bail!("Login failed with status: {}", response.status())
        }
    }
}
