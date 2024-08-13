pub mod base;
pub mod impls;
#[cfg(feature = "internals")]
pub mod internals;
#[cfg(not(feature = "internals"))]
pub(crate) mod internals;

pub mod extension;

#[cfg(test)]
mod test {

    use crate::{
        base::{
            app::{AppVisitor, Application},
            client::{Account, Client},
        },
        extension::calendar::{ApplicationCalendarExt, CalendarParser},
        impls::{
            apps::sso::{self, jwcas::JwcasApplication},
            client::DefaultClient,
            login::sso::SSOUniversalLogin,
        },
        internals::recursion::recursion_redirect_handle,
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
                recursion_redirect_handle(self.client.clone(), " url")
                    .await
                    .unwrap();
            }
        }

        tokio::spawn(async {
            let client = DefaultClient::new(Account::new("user", " password"));
            client.sso_universal_login().await.unwrap();
            let foo = client.visit::<Foo<_>>().await;
            let _ = client.visit::<JwcasApplication<_>>().await;
            foo.login().await;
        });
    }

    #[tokio::test]
    async fn calendar() {
        let client = DefaultClient::new(Account::default());
        client.sso_universal_login().await.unwrap().unwrap();
        let app = client.visit::<sso::jwcas::JwcasApplication<_>>().await;
        app.login().await.unwrap();
        println!(
            "{:?}",
            app.column_matrix_to_classinfo(app.get_classinfo_string_week().await.unwrap())
        );
    }
}
