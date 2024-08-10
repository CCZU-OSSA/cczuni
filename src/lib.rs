pub mod base;
pub mod impls;
pub mod internals;

#[cfg(test)]
mod test {
    use reqwest::Url;

    use crate::{
        base::{
            app::{AppVisitor, Application},
            client::{Account, Client},
        },
        impls::{client::DefaultClient, login::sso::SSOLogin},
        internals::recursion::recursion_cookies_handle,
    };
    #[tokio::test]
    async fn spawn_test() {
        struct Foo<C> {
            client: C,
        }

        impl<C: Client> Application<C> for Foo<C> {
            fn from_client(client: C) -> Self {
                Foo { client }
            }
        }

        impl<C: Client + Clone + Send> Foo<C> {
            async fn login(&self) {
                self.client
                    .reqwest_client()
                    .get("url")
                    .send()
                    .await
                    .unwrap();
                recursion_cookies_handle(
                    self.client.clone(),
                    " url",
                    &Url::parse("input").unwrap(),
                )
                .await
                .unwrap();
            }
        }

        tokio::spawn(async {
            let client = DefaultClient::new(Account::new("user", " password"));
            client.sso_login().await.unwrap();
            let foo = client.visit::<Foo<_>>();
            foo.login().await;
        });
    }
}
