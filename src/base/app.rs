use std::future::Future;

use super::client::Client;

pub trait Application<C: Client> {
    /// Sometimes need async to initialize the struct data
    #[allow(opaque_hidden_inferred_bound)]
    fn from_client(client: C) -> impl Future<Output = Self>;
}

pub trait AppVisitor<C: Client> {
    fn visit<T: Application<C>>(&self) -> impl Future<Output = T>;
}

impl<C: Client + Clone> AppVisitor<C> for C {
    async fn visit<T: Application<Self>>(&self) -> T {
        T::from_client(self.clone()).await
    }
}
