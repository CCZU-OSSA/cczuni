use std::future::Future;

use super::client::Client;

pub trait Application<C: Client> {
    /// Sometimes need async to initialize the struct data
    fn from_client(client: C) -> impl Future<Output = Self>;
    fn from_client_sync(_: C) -> Self
    where
        Self: Sized,
    {
        unimplemented!("Use the async version instead")
    }
}

pub trait AppVisitor<C: Client> {
    fn visit<T: Application<C>>(&self) -> impl Future<Output = T>;
    fn visit_sync<T: Application<C>>(&self) -> T {
        unimplemented!("Use the async version instead")
    }
}

impl<C: Client + Clone> AppVisitor<C> for C {
    async fn visit<T: Application<Self>>(&self) -> T {
        T::from_client(self.clone()).await
    }

    fn visit_sync<T: Application<Self>>(&self) -> T {
        T::from_client_sync(self.clone())
    }
}
