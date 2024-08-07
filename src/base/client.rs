use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;
use tokio::sync::Mutex;

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

pub trait Client {
    fn account(&self) -> Account;
    fn reqwest_client(&self) -> Arc<Mutex<reqwest::Client>>;
    fn cookies(&self) -> Arc<CookieStoreMutex>;
}
