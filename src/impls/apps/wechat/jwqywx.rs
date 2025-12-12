use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ORIGIN, REFERER};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base::{app::Application, client::Client},
    impls::apps::wechat::jwqywx_type::EvaluatableClass,
    internals::fields::{DEFAULT_HEADERS, WECHAT_APP_API},
};
use anyhow::{Context, Ok, Result};

use super::jwqywx_type::{CourseGrade, Exam, LoginUserData, Message, StudentPoint, Term};

pub struct JwqywxApplication<C> {
    client: C,
    headers: Arc<RwLock<HeaderMap>>,
    authorizationid: Arc<RwLock<Option<String>>>,
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
            authorizationid: Arc::new(RwLock::new(None)),
        }
    }
}

impl<C: Client> JwqywxApplication<C> {
    pub async fn login(&self) -> Result<Message<LoginUserData>> {
        let account = self.client.account();
        let response = self
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
            .await?;
        let text = response.text().await?;
        let message: Message<LoginUserData> = serde_json::from_str(&text)?;
        self.write_token(format!(
            "Bearer {}",
            message.token.clone().context("No token available")?
        ))
        .await;
        self.write_authorizationid(
            message
                .message
                .get(0)
                .context("Jwqywx Login Failed, No User Data!")?
                .id
                .clone(),
        )
        .await;

        Ok(message)
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

    async fn write_authorizationid(&self, id: String) {
        let mut authorizationid = self.authorizationid.write().await;
        *authorizationid = Some(id);
    }

    async fn get_authorizationid(&self) -> Result<String> {
        let authorizationid = self.authorizationid.read().await;
        authorizationid.clone().context("Not logged in")
    }

    pub async fn get_grades(&self) -> Result<Message<CourseGrade>> {
        Ok(self
            .client
            .reqwest_client()
            .post(format!("{}/api/cj_xh", WECHAT_APP_API))
            .headers(self.headers.read().await.clone())
            .json(&json!({
                "xh":self.get_authorizationid().await?,
            }))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_credits_and_rank(&self) -> Result<Message<StudentPoint>> {
        Ok(self
            .client
            .reqwest_client()
            .post(format!("{}/api/cj_xh_xfjd", WECHAT_APP_API))
            .headers(self.headers.read().await.clone())
            .json(&json!({
                "xh":self.get_authorizationid().await?,
            }))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn terms(&self) -> Result<Message<Term>> {
        Ok(self
            .client
            .reqwest_client()
            .get(format!("{}/api/xqall", WECHAT_APP_API))
            .send()
            .await?
            .json()
            .await?)
    }

    /// Get Term from [`JwqywxApplication::terms`]
    pub async fn get_exams(&self, term: String) -> Result<Message<Exam>> {
        Ok(self
            .client
            .reqwest_client()
            .post(format!("{}/api/ks_xs_kslb", WECHAT_APP_API))
            .headers(self.headers.read().await.clone())
            .json(&json!({
                "xq":term,
                "yhdm":self.client.account().user,
                "dm":"学分制考试",
                "yhid":self.get_authorizationid().await?,
            }))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_evaluatable_class(&self, term: String) -> Result<Message<EvaluatableClass>> {
        Ok(self
            .client
            .reqwest_client()
            .post(format!("{}/api/pj_xspj_kcxx", WECHAT_APP_API))
            .headers(self.headers.read().await.clone())
            .json(&json!({
                "pjxq":term,
                "xh":self.client.account().user,
                "yhid":self.get_authorizationid().await?,
            }))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn evaluate_class(
        &self,
        term: String,
        evaluatable_class: &EvaluatableClass,
        // 90
        overall_score: i32,
        // 100,80,100,80,100,80,
        scores: Vec<i32>,
        comments: String,
    ) -> Result<()> {
        let pjjg = scores
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",")
            + ",";
        self.client
            .reqwest_client()
            .post(format!("{}/api/pj_insert_xspj", WECHAT_APP_API))
            .headers(self.headers.read().await.clone())
            .json(&json!({
                "pjxq":term,
                "yhdm":self.client.account().user,
                "jsdm":evaluatable_class.teacher_code,
                "kcdm":evaluatable_class.course_code,
                "zhdf":overall_score,
                "pjjg":pjjg,
                "yjjy":comments,
                "yhid":self.get_authorizationid().await?,
            }))
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(feature = "calendar")]
pub mod calendar {
    use crate::{
        base::client::Client,
        extension::calendar::{CalendarParser, RawCourse, TermCalendarParser},
        impls::apps::wechat::jwqywx_type::{calendar::SerdeRowCourses, Message},
        internals::fields::WECHAT_APP_API,
    };
    use anyhow::{Context, Result};
    use serde_json::json;

    use super::JwqywxApplication;

    impl<C: Client> TermCalendarParser for JwqywxApplication<C> {
        async fn get_term_classinfo_week_matrix(
            &self,
            term: String,
        ) -> Result<Vec<Vec<RawCourse>>> {
            Ok(self
                .client
                .reqwest_client()
                .post(format!("{}/api/kb_xq_xh", WECHAT_APP_API))
                .headers(self.headers.read().await.clone())
                .json(&json!({
                    "xh":self.client.account().user,
                    "xq":term,
                    "yhid":self.get_authorizationid().await?,
                }))
                .send()
                .await?
                .json::<Message<SerdeRowCourses>>()
                .await?
                .message
                .into_iter()
                .map(|e| e.into())
                .collect())
        }
    }

    impl<C: Client> CalendarParser for JwqywxApplication<C> {
        async fn get_classinfo_week_matrix(&self) -> Result<Vec<Vec<RawCourse>>> {
            self.get_term_classinfo_week_matrix(
                self.terms()
                    .await?
                    .message
                    .first()
                    .context("No terms available")?
                    .term
                    .clone(),
            )
            .await
        }
    }
}
