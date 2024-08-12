use std::fmt::Display;

use reqwest::StatusCode;
use scraper::{ElementRef, Html, Selector};

use crate::base::app::Application;
use crate::base::client::Client;
use crate::base::typing::{EmptyOrErr, TorErr};
use crate::impls::services::sso_redirect::SSORedirect;

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

impl<C: Client + Clone> JwcasApplication<C> {
    /// will call login after [`Self::from_client`]
    pub async fn from_client_login(client: C) -> TorErr<Self> {
        let app = Self::from_client(client).await;
        app.login().await?;
        Ok(app)
    }

    /// Visit this Url will login in too.
    pub async fn login(&self) -> EmptyOrErr {
        let api = format!("{}/web_cas/web_cas_login_jwgl.aspx", self.root);
        self.client.sso_initialize_url(api.clone()).await;
        if let Ok(response) = self.client.reqwest_client().get(api).send().await {
            let redirect_url = response
                .headers()
                .get("location")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            self.client.sso_initialize_url(&redirect_url).await;
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
                self.client.sso_initialize_url(redirect_url).await;
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
        self.client.sso_initialize_url(&api).await;
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

#[cfg(feature = "calendar")]
pub mod calendar {
    use std::collections::HashMap;

    use regex::Regex;
    use scraper::{Html, Selector};
    use uuid::Uuid;

    use crate::base::client::Client;
    use crate::base::typing::TorErr;
    use crate::extension::calendar::{CalendarParser, ClassInfo};

    use super::JwcasApplication;
    impl<C: Client + Clone> CalendarParser for JwcasApplication<C> {
        async fn get_classinfo_vec(&self) -> TorErr<Vec<ClassInfo>> {
            let opt_text = self.get_classlist_html().await;
            if let None = opt_text {
                return Err("获取页面错误");
            }
            let text = opt_text.unwrap();
            let doc = Html::parse_document(&text);
            let tb_up_seletor = Selector::parse(r#"table[id="GVxkall"]"#).unwrap();
            let tb_dn_seletor = Selector::parse(r#"table[id="GVxkkb"]"#).unwrap();
            let tb_up_itemseletor =
                Selector::parse(r#"tr[class="dg1-item"] > td[width="20%"] > font"#).unwrap();
            let tb_dn_rowseletor = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
            let tb_dn_itemseletor = Selector::parse(r#"td[width="12%"] > font"#).unwrap();
            let class_namelist: Vec<String> = doc
                .select(&tb_up_seletor)
                .next()
                .unwrap()
                .select(&tb_up_itemseletor)
                .map(|e| e.inner_html())
                .collect();

            let row_matrix: Vec<Vec<String>> = doc
                .select(&tb_dn_seletor)
                .next()
                .unwrap()
                .select(&tb_dn_rowseletor)
                .map(|e| {
                    e.select(&tb_dn_itemseletor)
                        .map(|item| dbg!(item.inner_html()))
                        .collect()
                })
                .collect();
            let mut column_matrix: Vec<Vec<String>> = vec![];
            for i in 0..7 {
                let mut tmp: Vec<String> = vec![];
                for v in row_matrix.iter() {
                    if let Some(value) = v.get(i) {
                        tmp.push(value.clone())
                    } else {
                        return Err("课程表解析错误".into());
                    }
                }
                column_matrix.push(tmp.clone());
            }

            let mut course_info: HashMap<String, ClassInfo> = HashMap::new();

            for (day, courses) in column_matrix.iter().enumerate() {
                for (time, course_cb) in courses.iter().enumerate() {
                    let mut target: Vec<String> = course_cb
                        .split("/")
                        .filter(|v| !v.is_empty())
                        .map(|v| v.to_string())
                        .collect();
                    let targetlen = target.len();
                    for index in 0..targetlen {
                        let course = target[index].clone();
                        if course != "&nbsp;" {
                            let id = Uuid::new_v3(
                                &Uuid::NAMESPACE_DNS,
                                format!("{}{}", course, day).as_bytes(),
                            )
                            .to_string();

                            if !course_info.contains_key(&id) || time == 0 {
                                let nl: Vec<String> = class_namelist
                                    .iter()
                                    .filter(|e| course.starts_with(e.as_str()))
                                    .map(|e| e.clone())
                                    .collect();
                                if nl.is_empty() {
                                    if index < targetlen - 1 {
                                        target[index + 1] =
                                            format!("{}/{}", course.clone(), target[index + 1]);
                                        continue;
                                    }
                                    return Err("Unable to resolve course name correctly".into());
                                }

                                let classname = nl[0].clone();

                                let re =
                                    Regex::new(r#"(\S+)? *([单双]?) *((\d+-\d+,?)+)"#).unwrap();
                                let pattern = course.replace(&classname, "").trim().to_string();
                                let Some(data) = re.captures(pattern.as_str()) else {
                                    return Err("Course information parsing abnormal!".into());
                                }; //'X立德楼409  7-8,'

                                let info = ClassInfo::new(
                                    classname,
                                    match data.get(2).map_or("", |m| m.as_str()) {
                                        "单" => 1,
                                        "双" => 2,
                                        _ => 3,
                                    },
                                    day + 1,
                                    data.get(3)
                                        .map_or("", |m| m.as_str())
                                        .split(",")
                                        .filter(|e| !e.is_empty())
                                        .map(|e| e.to_string())
                                        .collect(),
                                    vec![time + 1],
                                    data.get(1)
                                        .map_or(String::new(), |m| m.as_str().to_string()),
                                );
                                course_info.insert(id, info);
                            } else {
                                course_info.get_mut(&id).unwrap().add_classtime(time + 1);
                            }
                        }
                    }
                }
            }

            Ok(course_info
                .values()
                .into_iter()
                .map(|e| e.clone())
                .collect())
        }
    }
}
