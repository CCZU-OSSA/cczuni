use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ORIGIN, REFERER};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base::{
        app::Application,
        client::Client,
        typing::{other_error, TorErr},
    },
    internals::fields::{DEFAULT_HEADERS, WECHAT_APP_API},
};

use super::jwqywx_type::{CourseGrade, LoginUserData, Message, StudentPoint, Term};

pub struct JwqywxApplication<C> {
    client: C,
    headers: Arc<RwLock<HeaderMap>>,
}

impl<C: Client> Application<C> for JwqywxApplication<C> {
    async fn from_client(client: C) -> Self {
        let mut header = DEFAULT_HEADERS.clone();
        header.insert(
            REFERER,
            HeaderValue::from_static("http://jwqywx.cczu.edu.cn/"),
        );
        header.insert(
            ORIGIN,
            HeaderValue::from_static("http://jwqywx.cczu.edu.cn"),
        );
        Self {
            client,
            headers: Arc::new(RwLock::new(header)),
        }
    }
}

impl<C: Client> JwqywxApplication<C> {
    pub async fn login(&self) -> TorErr<Message<LoginUserData>> {
        let account = self.client.account();
        let result = self
            .client
            .reqwest_client()
            .post(format!("{}/api/login", WECHAT_APP_API))
            .headers(DEFAULT_HEADERS.clone())
            .header("Referer", "http://jwqywx.cczu.edu.cn/")
            .header("Origin", "http://jwqywx.cczu.edu.cn")
            .json(&json!({
                "userid":account.user,
                "userpwd":account.password,
            }))
            .send()
            .await;
        if let Ok(response) = result {
            if let Ok(text) = response.text().await {
                let message = serde_json::from_str::<Message<LoginUserData>>(&text)?;
                self.write_token(format!("Bearer {}", message.token.clone().unwrap()))
                    .await;
                return Ok(message);
            }
        }
        Err(other_error("Jwqywx Login Failed"))
    }

    async fn write_token(&self, token: String) {
        let mut header = DEFAULT_HEADERS.clone();
        header.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
        header.insert(
            REFERER,
            HeaderValue::from_static("http://jwqywx.cczu.edu.cn/"),
        );
        header.insert(
            ORIGIN,
            HeaderValue::from_static("http://jwqywx.cczu.edu.cn"),
        );
        *self.headers.write().await = header;
    }

    pub async fn get_grades(&self) -> TorErr<Message<CourseGrade>> {
        let result = self
            .client
            .reqwest_client()
            .post(format!("{}/api/cj_xh", WECHAT_APP_API))
            .headers(self.headers.read().await.clone())
            .json(&json!({
                "xh":self.client.account().user,
            }))
            .send()
            .await;
        if let Ok(response) = result {
            return Ok(response.json().await.map_err(other_error)?);
        }
        Err(other_error("Request Failed"))
    }

    pub async fn get_points(&self) -> TorErr<Message<StudentPoint>> {
        let result = self
            .client
            .reqwest_client()
            .post(format!("{}/api/cj_xh_xfjdpm", WECHAT_APP_API))
            .headers(self.headers.read().await.clone())
            .json(&json!({
                "xh":self.client.account().user,
            }))
            .send()
            .await;
        if let Ok(response) = result {
            return Ok(response.json().await.map_err(other_error)?);
        }
        Err(other_error("Request Failed"))
    }

    pub async fn terms(&self) -> TorErr<Message<Term>> {
        let result = self
            .client
            .reqwest_client()
            .get(format!("{}/api/xqall", WECHAT_APP_API))
            .send()
            .await;
        if let Ok(response) = result {
            return Ok(response.json().await.map_err(other_error)?);
        }
        Err(other_error("Request Failed"))
    }
}

#[cfg(feature = "calendar")]
pub mod calendar {
    use serde_json::json;

    use crate::{
        base::{
            client::Client,
            typing::{other_error, TorErr},
        },
        extension::calendar::{
            row_matrix_to_classinfo, CalendarParser, ParsedCourse, TermCalendarParser,
        },
        impls::apps::wechat::jwqywx_type::{calendar::SerdeRowCourses, Message},
        internals::fields::WECHAT_APP_API,
    };

    use super::JwqywxApplication;

    impl<C: Client> TermCalendarParser for JwqywxApplication<C> {
        async fn get_term_classinfo_week_matrix(&self, term: String) -> TorErr<Vec<ParsedCourse>> {
            let result = self
                .client
                .reqwest_client()
                .post(format!("{}/api/kb_xq_xh", WECHAT_APP_API))
                .headers(self.headers.read().await.clone())
                .json(&json!({
                    "xh":self.client.account().user,
                    "xq":term,
                }))
                .send()
                .await;
            if let Ok(response) = result {
                let data: Message<SerdeRowCourses> = response.json().await.unwrap();
                return Ok(row_matrix_to_classinfo(
                    data.message.into_iter().map(|e| e.into()).collect(),
                )?);
            }
            Err(other_error("Get Class Info failed"))
        }
    }

    impl<C: Client> CalendarParser for JwqywxApplication<C> {
        async fn get_classinfo_week_matrix(&self) -> TorErr<Vec<ParsedCourse>> {
            self.get_term_classinfo_week_matrix(
                self.terms()
                    .await
                    .unwrap()
                    .message
                    .first()
                    .unwrap()
                    .term
                    .clone(),
            )
            .await
        }
    }
}
