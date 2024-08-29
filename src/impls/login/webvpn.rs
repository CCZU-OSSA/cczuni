use std::{collections::HashMap, io::ErrorKind};

use crate::{
    base::{client::Client, typing::TorErr},
    impls::login::sso_type::ElinkLoginInfo,
    internals::fields::{DEFAULT_HEADERS, ROOT_VPN},
};
use aes::{
    cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit},
    Aes128Enc,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use cbc::Encryptor;
use rand::Rng;
use reqwest::{cookie::Cookie, StatusCode};
pub type CbcAES128Enc = Encryptor<Aes128Enc>;

pub trait WebVPNLogin {
    fn webvpn_login(&self) -> impl std::future::Future<Output = TorErr<ElinkLoginInfo>>;
}

impl<C: Client> WebVPNLogin for C {
    async fn webvpn_login(&self) -> TorErr<ElinkLoginInfo> {
        let account = self.account();
        let url = format!("{}/enlink/sso/login/submit", ROOT_VPN);
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut rng = rand::thread_rng();
        let mut token = (0..16)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
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
            .unwrap();
        let encrypt_pwd = BASE64_STANDARD.encode(encrypt_buf);
        let mut data: HashMap<&'static str, String> = HashMap::new();
        data.insert("username", account.user);
        data.insert("password", encrypt_pwd);
        data.insert(
            "token",
            token.iter().map(|char| char.clone() as char).collect(),
        );
        data.insert("language", "zh-CN,zh;q=0.9,en;q=0.8".into());
        if let Ok(response) = self
            .reqwest_client()
            .post(url)
            .header("Refer", format!("{}/enlink/sso/login", ROOT_VPN))
            .header("Origin", ROOT_VPN)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .headers(DEFAULT_HEADERS.clone())
            .form(&data)
            .send()
            .await
        {
            if response.status() == StatusCode::FOUND {
                if let Some(cookie) = &response
                    .cookies()
                    .filter(|cookie| cookie.name() == "clientInfo")
                    .collect::<Vec<Cookie>>()
                    .first()
                {
                    let json =
                        String::from_utf8(BASE64_STANDARD.decode(cookie.value()).unwrap()).unwrap();
                    return Ok(serde_json::from_str(json.as_str())?);
                }
            }
        };
        Err(tokio::io::Error::new(
            ErrorKind::ConnectionAborted,
            "普通登录失败，请检查账号密码是否错误...",
        ))
    }
}
