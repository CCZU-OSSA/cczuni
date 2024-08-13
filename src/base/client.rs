use reqwest_cookie_store::CookieStoreMutex;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

/// You must decide what account to use to invoke different method!
#[derive(Debug, Clone)]
pub struct Account {
    pub user: String,
    pub password: String,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            user: Default::default(),
            password: Default::default(),
        }
    }
}

impl Account {
    pub fn new(user: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            user: user.into(),
            password: password.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Property {
    String(String),
    I32(i32),
    Bool(bool),
    Str(&'static str),
}

impl Property {
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Property::Bool(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_bool_unwrap(&self) -> bool {
        self.get_bool().unwrap()
    }

    pub fn get_i32(&self) -> Option<i32> {
        match self {
            Property::I32(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_i32_unwrap(&self) -> i32 {
        self.get_i32().unwrap()
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Property::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_string_unwrap(&self) -> String {
        self.get_string().unwrap()
    }

    pub fn get_str(&self) -> Option<&'static str> {
        match self {
            Property::Str(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_str_unwrap(&self) -> &'static str {
        self.get_str().unwrap()
    }
}

pub trait Client {
    fn account(&self) -> Account;
    fn reqwest_client(&self) -> reqwest::Client;
    fn cookies(&self) -> Arc<CookieStoreMutex>;
    fn properties(&self) -> Arc<RwLock<HashMap<&'static str, Property>>>;
}
