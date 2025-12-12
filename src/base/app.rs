use std::future::Future;

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
}

impl<C: Client> AppVisitor<C> for C {
    async fn visit<T: Application<Self>>(&self) -> T {
        T::from_client(self).await
    }
}
