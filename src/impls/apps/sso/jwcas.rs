use std::fmt::Display;

use reqwest::StatusCode;
use scraper::{ElementRef, Html, Selector};

use crate::base::app::Application;
use crate::base::client::Client;
use crate::base::typing::{EmptyOrErr, TorErr};
use crate::impls::services::sso_redirect::SSORedirect;
use crate::internals::error::ERROR_REQUEST_FAILED;
use crate::internals::recursion::recursion_redirect_handle;

use super::jwcas_type::GradeData;

pub struct JwcasApplication<C> {
    pub client: C,
    pub root: String,
}

impl<C: Client + Clone> Application<C> for JwcasApplication<C> {
    async fn from_client(client: C) -> Self {
        Self {
            client: client.clone(),
            root: client.sso_redirect("http://219.230.159.132").await,
        }
    }
}

impl<C: Client + Clone + Send> JwcasApplication<C> {
    /// will call login after [`Self::from_client`]
    pub async fn from_client_login(client: C) -> TorErr<Self> {
        let app = Self::from_client(client).await;
        app.login().await?;
        Ok(app)
    }

    /// Visit this Url will login in too.
    pub async fn login(&self) -> EmptyOrErr {
        let api = format!("{}/web_cas/web_cas_login_jwgl.aspx", self.root);
        recursion_redirect_handle(self.client.clone(), &api)
            .await
            .unwrap();
        Ok(())
    }

    pub async fn get_classlist_html(&self) -> Option<String> {
        self.get_html("/web_jxrw/cx_kb_xsgrkb.aspx").await
    }

    pub async fn get_gradelist_html(&self) -> Option<String> {
        self.get_html("/web_cjgl/cx_cj_jxjhcj_xh.aspx").await
    }

    pub async fn get_html(&self, service: impl Display) -> Option<String> {
        let api = format!("{}{}", self.root, service);

        if let Ok(response) = self.client.reqwest_client().get(api).send().await {
            if response.status() != StatusCode::OK {
                None
            } else {
                Some(response.text().await.unwrap())
            }
        } else {
            None
        }
    }

    pub async fn get_gradeinfo_vec(&self) -> TorErr<Vec<GradeData>> {
        if let Some(text) = self.get_gradelist_html().await {
            let tb_up = Selector::parse(r#"table[id="GVkbk"]"#).unwrap();
            let selector = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
            let dom = Html::parse_document(&text);
            Ok(dom
                .select(&tb_up)
                .next()
                .unwrap()
                .select(&selector)
                .map(|e| {
                    let childs: Vec<ElementRef> = e.child_elements().collect();
                    GradeData {
                        name: extract_string(childs.get(5)),
                        point: extract_string(childs.get(8)),
                        grade: extract_string(childs.get(9)),
                    }
                })
                .collect())
        } else {
            Err(ERROR_REQUEST_FAILED)
        }
    }
}

fn extract_string(element: Option<&ElementRef>) -> String {
    if let Some(element) = element {
        element.text().next().unwrap().to_string()
    } else {
        String::new()
    }
}

#[cfg(feature = "calendar")]
pub mod calendar {

    use scraper::{Html, Selector};

    use crate::base::client::Client;
    use crate::base::typing::TorErr;
    use crate::extension::calendar::CalendarParser;
    use crate::internals::error::ERROR_PAGE_CONTENT;

    use super::JwcasApplication;
    impl<C: Client + Clone + Send> CalendarParser for JwcasApplication<C> {
        async fn get_classinfo_week_matrix(&self) -> TorErr<Vec<Vec<String>>> {
            let text = self.get_classlist_html().await.ok_or(ERROR_PAGE_CONTENT)?;

            let doc = Html::parse_document(&text);
            let tb_dn_seletor = Selector::parse(r#"table[id="GVxkkb"]"#).unwrap();

            let tb_dn_rowseletor = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
            let tb_dn_itemseletor = Selector::parse(r#"td"#).unwrap();

            let row_matrix: Vec<Vec<String>> = doc
                .select(&tb_dn_seletor)
                .next()
                .unwrap()
                .select(&tb_dn_rowseletor)
                .map(|e| {
                    let mut items: Vec<String> = e
                        .select(&tb_dn_itemseletor)
                        .map(|item| item.inner_html())
                        .collect();
                    items.remove(0);
                    items
                })
                .collect();

            Ok(row_matrix)
        }
    }
}
