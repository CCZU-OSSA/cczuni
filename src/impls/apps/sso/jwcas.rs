use std::fmt::Display;

use reqwest::StatusCode;
use scraper::{ElementRef, Html, Selector};

use crate::base::app::Application;
use crate::base::client::Client;
use crate::base::typing::{other_error, EmptyOrErr, TorErr};
use crate::impls::services::sso_redirect::SSORedirect;
use crate::internals::recursion::recursion_redirect_handle;

use super::jwcas_type::GradeData;

/// Call `sso_login` before using this application!
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
            .map_err(other_error)?;
        Ok(())
    }

    pub async fn get_classlist_html(&self) -> TorErr<String> {
        self.get_html("/web_jxrw/cx_kb_xsgrkb.aspx").await
    }

    pub async fn get_gradelist_html(&self) -> TorErr<String> {
        self.get_html("/web_cjgl/cx_cj_jxjhcj_xh.aspx").await
    }

    pub async fn get_html(&self, service: impl Display) -> TorErr<String> {
        let api = format!("{}{}", self.root, service);

        if let Ok(response) = self.client.reqwest_client().get(api).send().await {
            if response.status() == StatusCode::OK {
                return Ok(response.text().await.unwrap());
            }
        }

        Err(other_error(format!("Get {service} failed")))
    }

    pub async fn get_gradeinfo_vec(&self) -> TorErr<Vec<GradeData>> {
        let text = self.get_gradelist_html().await?;
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
    use std::collections::HashMap;

    use scraper::{Html, Selector};

    use crate::base::client::Client;
    use crate::base::typing::{other_error, TorErr};
    use crate::extension::calendar::{CalendarParser, RawCourse};

    use super::JwcasApplication;
    impl<C: Client + Clone + Send> CalendarParser for JwcasApplication<C> {
        async fn get_classinfo_week_matrix(&self) -> TorErr<Vec<Vec<RawCourse>>> {
            let text = self.get_classlist_html().await?;
            let doc = Html::parse_document(&text);

            // used to select teacher
            let tb_up_rowseletor = Selector::parse(r#"table[id="GVxkall"]"#).unwrap();

            // used to select course
            let tb_dn_seletor: Selector = Selector::parse(r#"table[id="GVxkkb"]"#).unwrap();

            let tb_dg1_itemseletor = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
            let tb_tdseletor = Selector::parse(r#"td"#).unwrap();
            let tb_td_with_fontseletor = Selector::parse(r#"td > font"#).unwrap();
            let mut teachers = HashMap::new();
            doc.select(&tb_up_rowseletor)
                .next()
                .ok_or(other_error("Select Teacher Failed"))?
                .select(&tb_dg1_itemseletor)
                .for_each(|e| {
                    let items: Vec<String> = e
                        .select(&tb_td_with_fontseletor)
                        .map(|item| item.inner_html().trim().to_string())
                        .collect();
                    if !items.is_empty() {
                        teachers.insert(items[1].clone(), items[5].clone());

                        return;
                    }
                    let items: Vec<String> = e
                        .select(&tb_tdseletor)
                        .map(|item| item.inner_html().trim().to_string())
                        .collect();
                    teachers.insert(items[1].clone(), items[5].clone());
                });

            Ok(doc
                .select(&tb_dn_seletor)
                .next()
                .ok_or(other_error("Select Course Failed"))?
                .select(&tb_dg1_itemseletor)
                .map(|e| {
                    let mut items: Vec<String> = e
                        .select(&tb_td_with_fontseletor)
                        .map(|item| item.inner_html())
                        .collect();
                    if !items.is_empty() {
                        items.remove(0);

                        return items;
                    }

                    let mut items: Vec<String> = e
                        .select(&tb_tdseletor)
                        .map(|item| item.inner_html())
                        .collect();
                    items.remove(0);
                    items
                })
                .map(|courses| {
                    courses
                        .into_iter()
                        .map(|course| {
                            let teacher = teachers
                                .get(
                                    course
                                        .split(" ")
                                        .collect::<Vec<&str>>()
                                        .first()
                                        .cloned()
                                        .unwrap_or(""),
                                )
                                .cloned()
                                .unwrap_or(String::new());

                            RawCourse { course, teacher }
                        })
                        .collect()
                })
                .collect())
        }
    }
}
