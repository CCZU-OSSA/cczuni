use std::sync::Arc;

use reqwest::redirect::Policy;
use reqwest_cookie_store::CookieStoreMutex;
use tokio::sync::Mutex;

use crate::base::client::{Account, Client};

#[derive(Debug, Clone)]
pub struct DefaultClient {
    account: Account,
    client: Arc<Mutex<reqwest::Client>>,
    cookies: Arc<CookieStoreMutex>,
}

impl DefaultClient {
    pub fn new(account: Account) -> Self {
        let cookies = Arc::new(CookieStoreMutex::default());
        Self {
            account,
            client: Arc::new(Mutex::new(
                reqwest::Client::builder()
                    .cookie_provider(cookies.clone())
                    .redirect(Policy::none())
                    .build()
                    .unwrap(),
            )),
            cookies,
        }
    }
}

impl Client for DefaultClient {
    fn account(&self) -> Account {
        self.account.clone()
    }

    fn reqwest_client(&self) -> Arc<Mutex<reqwest::Client>> {
        self.client.clone()
    }

    fn cookies(&self) -> Arc<CookieStoreMutex> {
        self.cookies.clone()
    }
}
