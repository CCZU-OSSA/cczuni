use reqwest_cookie_store::CookieStoreMutex;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Account {
    pub user: String,
    pub password: String,
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
}

pub trait Client {
    fn account(&self) -> Account;
    fn reqwest_client(&self) -> reqwest::Client;
    fn cookies(&self) -> Arc<CookieStoreMutex>;
    fn properties(&self) -> Arc<RwLock<HashMap<&'static str, Property>>>;
}
