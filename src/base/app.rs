use std::future::Future;

use anyhow::Result;

use super::client::Client;

pub trait Application<C: Client> {
    /// Sometimes need async to initialize the struct data
    fn from_client(client: &C) -> impl Future<Output = Self>;
    /// Default constructor for flexibility
    fn new() -> Self
    where
        Self: Sized,
    {
        unimplemented!("Provide a default implementation if needed")
    }
}

pub trait AppVisitor<C: Client> {
    fn visit<T: Application<C>>(&self) -> impl Future<Output = T>;
    fn try_restore<T: CachedApplication<C>>(&self) -> impl Future<Output = Option<T>>;
}

impl<C: Client> AppVisitor<C> for C {
    async fn visit<T: Application<Self>>(&self) -> T {
        T::from_client(self).await
    }

    fn try_restore<T: CachedApplication<C>>(&self) -> impl Future<Output = Option<T>> {
        T::try_restore(self)
    }
}

pub trait CachedApplication<C: Client>: Application<C> {
    fn cache(&self) -> impl Future<Output = Result<()>>;
    fn try_restore(client: &C) -> impl Future<Output = Option<Self>>
    where
        Self: Sized;
}
