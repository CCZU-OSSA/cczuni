use std::{collections::HashMap, sync::Arc};

use reqwest::redirect::Policy;
use reqwest_cookie_store::CookieStoreMutex;
use tokio::sync::RwLock;

use crate::base::client::{Account, Client, Property};

#[derive(Debug, Clone)]
pub struct DefaultClient {
    account: Account,
    client: reqwest::Client,
    cookies: Arc<CookieStoreMutex>,
    properties: Arc<RwLock<HashMap<&'static str, Property>>>,
}

impl Default for DefaultClient {
    fn default() -> Self {
        Self::new(Account::default())
    }
}

impl DefaultClient {
    pub fn new(account: Account) -> Self {
        let cookies = Arc::new(CookieStoreMutex::default());
        Self {
            account,
            client: reqwest::Client::builder()
                .cookie_provider(cookies.clone())
                .redirect(Policy::none())
                .build()
                .unwrap(),
            cookies,
            properties: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn account(user: impl Into<String>, password: impl Into<String>) -> Self {
        Self::new(Account::new(user, password))
    }

    pub fn user(user: impl Into<String>) -> Self {
        Self::new(Account::new(user, ""))
    }
}

impl Client for DefaultClient {
    fn account(&self) -> Account {
        self.account.clone()
    }

    fn reqwest_client(&self) -> reqwest::Client {
        self.client.clone()
    }

    fn cookies(&self) -> Arc<CookieStoreMutex> {
        self.cookies.clone()
    }

    fn properties(&self) -> Arc<RwLock<HashMap<&'static str, Property>>> {
        self.properties.clone()
    }
}
