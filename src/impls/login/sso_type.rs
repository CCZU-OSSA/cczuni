use std::collections::HashMap;

use reqwest::Response;
use serde::Deserialize;

use crate::base::client::Property;

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkLoginInfo {
    pub username: String,
    pub sid: String,
    #[serde(rename = "userId")]
    pub userid: String,
    #[serde(rename = "loginKey")]
    pub loginkey: String,
}

#[derive(Debug, Clone)]
pub enum SSOLoginConnectType {
    WEBVPN,
    COMMON,
}

impl SSOLoginConnectType {
    #[inline(always)]
    pub fn key() -> &'static str {
        "login-connect-type"
    }
}

impl Into<Property> for SSOLoginConnectType {
    fn into(self) -> Property {
        Property::Str(match self {
            SSOLoginConnectType::COMMON => "common",
            SSOLoginConnectType::WEBVPN => "webvpn",
        })
    }
}

impl Into<SSOLoginConnectType> for Property {
    fn into(self) -> SSOLoginConnectType {
        match self {
            Property::Str(data) => match data {
                "common" => SSOLoginConnectType::COMMON,
                "webvpn" => SSOLoginConnectType::WEBVPN,
                _ => panic!("Unknown Login Connect Type"),
            },
            _ => panic!("Wrong Property Type, expect `Property::String`"),
        }
    }
}

impl Into<SSOLoginConnectType> for HashMap<&'static str, Property> {
    fn into(self) -> SSOLoginConnectType {
        self.get(SSOLoginConnectType::key())
            .expect("Can't get LoginConnectType! Need Login?")
            .clone()
            .into()
    }
}

pub struct SSOUniversalLoginInfo {
    pub response: Response,
    pub login_connect_type: SSOLoginConnectType,
}
