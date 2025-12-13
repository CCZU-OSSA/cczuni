#[cfg(feature = "lru-client")]
use lru::LruCache;
use reqwest::redirect::Policy;
use reqwest_cookie_store::CookieStoreMutex;
#[cfg(feature = "lru-client")]
use std::num::NonZeroUsize;
#[cfg(feature = "lru-client")]
use std::sync::LazyLock;
use std::{collections::HashMap, sync::Arc};
#[cfg(feature = "lru-client")]
use tokio::sync::Mutex;
use tokio::sync::RwLock;

use crate::base::client::{Account, Client, Property};

#[cfg(feature = "lru-client")]
static CACHE_SIZE: LazyLock<NonZeroUsize> = LazyLock::new(|| {
    std::env::var("CCZUNI_CACHE_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3)
        .try_into()
        .unwrap_or(NonZeroUsize::new(3).unwrap())
});

#[cfg(feature = "lru-client")]
static CLIENT_CACHE: LazyLock<Mutex<LruCache<Account, DefaultClient>>> =
    LazyLock::new(|| Mutex::new(LruCache::new(*CACHE_SIZE)));

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

    pub fn iccard(card: impl Into<String>) -> Self {
        Self::new(Account::new(card, ""))
    }

    /// 使用 LRU 缓存创建或复用 DefaultClient 实例
    /// 对于相同 Account，返回缓存的单例；否则创建新实例并缓存
    #[cfg(feature = "lru-client")]
    pub async fn lru_new(account: Account) -> Self {
        let mut cache = CLIENT_CACHE.lock().await;
        if let Some(client) = cache.get(&account) {
            return client.clone();
        }
        let client = Self::new(account.clone());
        cache.put(account, client.clone());
        client
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
