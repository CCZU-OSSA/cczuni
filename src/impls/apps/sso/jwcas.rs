use std::fmt::Display;

use scraper::{ElementRef, Html, Selector};

use crate::base::client::Client;
use crate::impls::login::sso::parse_hidden_values;
use crate::impls::services::sso_redirect::SSORedirect;
use crate::internals::recursion::recursion_redirect_handle;
use crate::{base::app::Application, impls::apps::sso::jwcas_type::PlanData};
use anyhow::{Context, Result};

use super::jwcas_type::GradeData;

/// Call `sso_login` before using this application!
pub struct JwcasApplication<C> {
    pub client: C,
    pub root: String,
}

impl<C: Client + Clone> Application<C> for JwcasApplication<C> {
    async fn from_client(client: &C) -> Self {
        Self {
            client: client.clone(),
            root: client.sso_redirect("http://219.230.159.132").await,
        }
    }
}

impl<C: Client + Clone + Send> JwcasApplication<C> {
    /// will call login after [`Self::from_client`]
    pub async fn from_client_login(client: C) -> Result<Self> {
        let app = Self::from_client(&client).await;
        app.login().await?;
        Ok(app)
    }

    /// Visit this Url will login in too.
    pub async fn login(&self) -> Result<()> {
        let api = format!("{}/web_cas/web_cas_login_jwgl.aspx", self.root);
        recursion_redirect_handle(self.client.clone(), &api).await?;
        Ok(())
    }

    pub async fn get_classlist_html(&self) -> Result<String> {
        self.get_html("/web_jxrw/cx_kb_xsgrkb.aspx").await
    }

    pub async fn get_gradelist_html(&self) -> Result<String> {
        self.get_html("/web_cjgl/cx_cj_jxjhcj_xh.aspx").await
    }

    pub async fn get_plan_html(&self) -> Result<String> {
        let index = self.get_html("/web_jxjh/jxjh_cx.aspx").await.unwrap();
        let hiddens = parse_hidden_values(index.as_str()).unwrap();
        let __ddnj = Selector::parse(r#"select[id="DDnj"]"#).unwrap();
        let __txtcxxq = Selector::parse(r#"input[id="Txtcxxq"]"#).unwrap();
        let dom = Html::parse_document(&index);
        let form = [
            ("ScriptManager1","UpdatePanel2|Gvzydm"),
            ("__EVENTTARGET","Gvzydm"),
            ("__EVENTARGUMENT","CmdWh$0"),
            ("__VIEWSTATE",hiddens.get("__VIEWSTATE").unwrap()),
            ("__VIEWSTATEGENERATOR",hiddens.get("__VIEWSTATEGENERATOR").unwrap()),
            ("__VIEWSTATEENCRYPTED",hiddens.get("__VIEWSTATEENCRYPTED").unwrap()),
            ("Txtcxxq",dom.select(&__txtcxxq)
                .next()
                .context("Get Txtcxxq Failed")?
                .value()
                .attr("value")
                .unwrap_or("")),
            ("DDnj",dom.select(&__ddnj)
                .next()
                .context("Get DDnj Failed")?
                .child_elements()
                .next()
                .unwrap()
                .value()
                .attr("value")
                .unwrap_or("")),
            ("Txtzyxx",""),
            ("__ASYNCPOST","false"),
            ];
        self.post_html("/web_jxjh/jxjh_cx.aspx", &form).await
    }

    pub async fn get_html(&self, service: impl Display) -> Result<String> {
        let api = format!("{}{}", self.root, service);
        Ok(self
            .client
            .reqwest_client()
            .get(api)
            .send()
            .await?
            .text()
            .await?)
    }

    pub async fn post_html(&self, service: impl Display, form: &[(&str, &str)]) -> Result<String> {
        let api = format!("{}{}", self.root, service);
        Ok(self
            .client
            .reqwest_client()
            .post(api)
            .form(form)
            .send()
            .await?
            .text()
            .await?)
    }

    pub async fn get_plan_vec(&self) -> Result<Vec<PlanData>> {
        let text = self.get_plan_html().await?;
        let tb_up = Selector::parse(r#"table[id="GVjxjh"]"#).unwrap();
        let selector = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
        let dom = Html::parse_document(&text);
        Ok(dom
            .select(&tb_up)
            .next()
            .context("Select Grade Table Failed")?
            .select(&selector)
            .map(|e| {
                let childs: Vec<ElementRef> = e.child_elements().collect();
                PlanData {
                    term: extract_string(childs.get(1)),
                    code: extract_string(childs.get(2)),
                    name: extract_string(childs.get(3)),
                    category: extract_string(childs.get(4)),
                    period: extract_string(childs.get(5)),
                    credit: extract_string(childs.get(6)),
                    exam: extract_string(childs.get(7)),
                    exp_period: extract_string(childs.get(8)),
                    exp_credit: extract_string(childs.get(9)),
                    practice_period: extract_string(childs.get(10)),
                    specialization: extract_string(childs.get(11)),
                    faculty: extract_string(childs.get(12)),
                }
            })
            .collect())
    }

    pub async fn get_gradeinfo_vec(&self) -> Result<Vec<GradeData>> {
        let text = self.get_gradelist_html().await?;
        let tb_up = Selector::parse(r#"table[id="GVkbk"]"#).unwrap();
        let selector = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
        let dom = Html::parse_document(&text);
        Ok(dom
            .select(&tb_up)
            .next()
            .context("Select Grade Table Failed")?
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

    use anyhow::{Context, Result};
    use scraper::{Html, Selector};

    use crate::base::client::Client;
    use crate::extension::calendar::{CalendarParser, RawCourse};

    use super::JwcasApplication;
    impl<C: Client + Clone + Send> CalendarParser for JwcasApplication<C> {
        async fn get_classinfo_week_matrix(&self) -> Result<Vec<Vec<RawCourse>>> {
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
                .context("Select Teacher Failed")?
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
                .context("Select Course Failed")?
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
