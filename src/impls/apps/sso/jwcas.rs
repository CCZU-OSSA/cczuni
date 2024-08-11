use std::fmt::Display;

use reqwest::StatusCode;
use scraper::{ElementRef, Html, Selector};

use crate::base::app::Application;
use crate::base::client::Client;
use crate::base::typing::EmptyOrErr;
use crate::impls::redirect::Redirect;

use super::jwcas_type::GradeData;

pub struct JwcasApplication<C> {
    pub client: C,
    pub root: String,
}

impl<C: Client + Clone> Application<C> for JwcasApplication<C> {
    async fn from_client(client: C) -> Self {
        Self {
            client: client.clone(),
            root: client.redirect("http://219.230.159.132").await,
        }
    }
}

impl<C: Client> JwcasApplication<C> {
    pub async fn login(&self) -> EmptyOrErr {
        let api = format!("{}/web_cas/web_cas_login_jwgl.aspx", self.root);
        self.client.initialize_url(api.clone()).await;
        if let Ok(response) = self.client.reqwest_client().get(api).send().await {
            let redirect_url = response
                .headers()
                .get("location")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            self.client.initialize_url(&redirect_url).await;
            if let Ok(response) = self.client.reqwest_client().get(redirect_url).send().await {
                if response.status() != StatusCode::FOUND {
                    return Err("账户认证失败，请检查登录");
                }
                let redirect_url = response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()
                    .unwrap();
                self.client.initialize_url(redirect_url).await;
                let _ = self.client.reqwest_client().get(redirect_url).send().await;
            }
        }
        Ok(())
    }

    pub async fn get_classlist_html(&self) -> Option<String> {
        self.get_api_html("/web_jxrw/cx_kb_xsgrkb.aspx").await
    }

    pub async fn get_gradelist_html(&self) -> Option<String> {
        self.get_api_html("/web_cjgl/cx_cj_jxjhcj_xh.aspx").await
    }

    pub async fn get_api_html(&self, service: impl Display) -> Option<String> {
        let api = format!("{}{}", self.root, service);
        self.client.initialize_url(&api).await;
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

    pub async fn get_gradeinfo_vec(&self) -> Result<Vec<GradeData>, String> {
        if let Some(text) = self.get_gradelist_html().await {
            let selector = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
            let dom = Html::parse_document(&text);
            Ok(dom
                .select(&selector)
                .map(|e| {
                    let childs: Vec<ElementRef> = e.child_elements().collect();
                    GradeData {
                        name: extract_string(childs.get(5).unwrap()),
                        point: extract_string(childs.get(8).unwrap()),
                        grade: extract_string(childs.get(9).unwrap()),
                    }
                })
                .collect())
        } else {
            Err("获取页面失败".into())
        }
    }
}

fn extract_string(element: &ElementRef) -> String {
    element.text().next().unwrap().to_string()
}
