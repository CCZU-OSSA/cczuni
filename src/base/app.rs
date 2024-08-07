use super::client::Client;

pub trait Application<C: Client> {
    fn from_client(client: C) -> Self;
}

pub trait AppVisitor<C: Client> {
    fn visit<T: Application<C>>(&self) -> T;
}

impl<C: Client + Clone> AppVisitor<C> for C {
    fn visit<T: Application<Self>>(&self) -> T {
        T::from_client(self.clone())
    }
}
