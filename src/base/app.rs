use super::client::Client;

pub trait Application {
    fn from_client(client: impl Client) -> Self;
}
