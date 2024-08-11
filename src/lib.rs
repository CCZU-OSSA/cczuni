pub mod base;
pub mod impls;
#[cfg(feature = "internals")]
pub mod internals;
#[cfg(not(feature = "internals"))]
pub(crate) mod internals;

#[cfg(test)]
mod test {
    use reqwest::Url;

    use crate::{
        base::{
            app::{AppVisitor, Application},
            client::{Account, Client},
        },
        impls::{client::DefaultClient, login::sso::SSOUniversalLogin},
        internals::recursion::recursion_cookies_handle,
    };
    #[tokio::test]
    async fn spawn_test() {
        struct Foo<C> {
            client: C,
        }

        impl<C: Client> Application<C> for Foo<C> {
            async fn from_client(client: C) -> Self {
                Self { client }
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
            client.sso_universal_login().await.unwrap();
            let foo = client.visit::<Foo<_>>().await;
            foo.login().await;
        });
    }
}
