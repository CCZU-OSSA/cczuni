use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base::{app::Application, client::Client},
    internals::fields::{DEFAULT_HEADERS, WECHAT_APP_API},
};

use super::jwqywx_type::{CourseGrade, LoginUserData, Message, StudentPoint};

pub struct JwqywxApplication<C> {
    client: C,
    token: Arc<RwLock<String>>,
}

impl<C: Client> Application<C> for JwqywxApplication<C> {
    async fn from_client(client: C) -> Self {
        Self {
            client,
            token: Arc::new(RwLock::new(String::new())),
        }
    }
}

impl<C: Client> JwqywxApplication<C> {
    pub async fn login(&self) -> Option<Message<LoginUserData>> {
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
                let message = serde_json::from_str::<Message<LoginUserData>>(&text).unwrap();
                {
                    *self.token.write().await =
                        format!("Bearer {}", message.token.clone().unwrap());
                    return Some(message);
                }
            }
        }

        None
    }

    async fn headers(&self) -> HeaderMap {
        let mut header = DEFAULT_HEADERS.clone();
        header.insert(
            "Authorization",
            HeaderValue::from_str(self.token.read().await.clone().as_str()).unwrap(),
        );
        header.insert(
            "Referer",
            HeaderValue::from_static("http://jwqywx.cczu.edu.cn/"),
        );
        header.insert(
            "Origin",
            HeaderValue::from_static("http://jwqywx.cczu.edu.cn"),
        );
        header
    }

    pub async fn get_grades(&self) -> Option<Message<CourseGrade>> {
        let result = self
            .client
            .reqwest_client()
            .post(format!("{}/api/cj_xh", WECHAT_APP_API))
            .headers(self.headers().await)
            .json(&json!({
                "xh":self.client.account().user,
            }))
            .send()
            .await;
        if let Ok(response) = result {
            let message = response.json::<Message<CourseGrade>>().await.unwrap();

            return Some(message);
        }
        None
    }

    pub async fn get_points(&self) -> Option<Message<StudentPoint>> {
        let result = self
            .client
            .reqwest_client()
            .post(format!("{}/api/cj_xh_xfjdpm", WECHAT_APP_API))
            .headers(self.headers().await)
            .json(&json!({
                "xh":self.client.account().user,
            }))
            .send()
            .await;
        if let Ok(response) = result {
            let message = response.json::<Message<StudentPoint>>().await.unwrap();

            return Some(message);
        }
        None
    }
}

#[cfg(feature = "calendar")]
pub mod calendar {
    use crate::{
        base::{client::Client, typing::TorErr},
        extension::calendar::CalendarParser,
    };

    use super::JwqywxApplication;

    impl<C: Client> CalendarParser for JwqywxApplication<C> {
        /// Only get the latest data
        async fn get_classinfo_string_week(&self) -> TorErr<Vec<Vec<String>>> {
            todo!()
        }
    }
}
