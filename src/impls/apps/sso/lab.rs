use std::collections::HashMap;

use crate::{
    base::{app::Application, client::Client},
    impls::login::sso::SSOUniversalLogin,
    internals::fields::DEFAULT_HEADERS,
};
use anyhow::Result;

static LABAPP_ROOT: &'static str = "https://sysaqgl.cczu.edu.cn";

pub struct LabApplication<C> {
    client: C,
}

impl<C: Client> Application<C> for LabApplication<C> {
    async fn from_client(client: C) -> Self {
        Self { client }
    }
}

impl<C: Client + Clone + Send + Sync> LabApplication<C> {
    /// Support LAN/WAN
    pub async fn exam_login(&self) -> Result<()> {
        self.client
            .sso_service_login(format!("{LABAPP_ROOT}/labexam/examIDSLogin.php"))
            .await?;

        Ok(())
    }

    pub async fn exam_increase_thirty_secs(&self) -> Result<LabExamStudyInfo> {
        let api = format!("{LABAPP_ROOT}/labexam/exam_xuexi_online.php");
        let mut params = HashMap::new();
        params.insert("cmd", "xuexi_online");

        Ok(self
            .client
            .reqwest_client()
            .post(api)
            .headers(DEFAULT_HEADERS.clone())
            .form(&params)
            .send()
            .await?
            .json()
            .await?)
    }
}

#[derive(serde::Deserialize)]
pub struct LabExamStudyInfo {
    pub status: i32,
    #[serde(rename = "shichang")]
    pub total: String,
}
