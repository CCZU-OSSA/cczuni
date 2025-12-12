use std::{collections::HashMap, future::Future};

use crate::{
    base::client::Client,
    internals::fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN, ROOT_VPN},
};
use anyhow::Result;
use reqwest::StatusCode;

use super::webvpn_type::{
    ElinkProxyData, ElinkServiceData, ElinkServiceInfoData, ElinkUserInfoData, Message,
};

/// Must be used in WebVPN mode
///
/// Get the `user_id` from [`crate::impls::login::sso_type::ElinkLoginInfo`]
pub trait WebVPNService {
    fn webvpn_available(&self) -> impl Future<Output = bool>;

    fn webvpn_get_user_info(
        &self,
        user_id: impl Into<String>,
    ) -> impl Future<Output = Result<Message<ElinkUserInfoData>>>;
    fn webvpn_get_tree_with_service(
        &self,
        user_id: impl Into<String>,
    ) -> impl Future<Output = Result<Message<ElinkServiceInfoData>>>;
    fn webvpn_get_service_by_user(
        &self,
        user_id: impl Into<String>,
    ) -> impl Future<Output = Result<Message<Vec<ElinkServiceData>>>>;
    fn webvpn_get_visit_service_by_user(
        &self,
        user_id: impl Into<String>,
    ) -> impl Future<Output = Result<Message<Vec<ElinkServiceData>>>>;
    fn webvpn_get_proxy_service(
        &self,
        user_id: impl Into<String>,
    ) -> impl Future<Output = Result<Message<ElinkProxyData>>>;
}

impl<C: Client> WebVPNService for C {
    async fn webvpn_get_user_info(
        &self,
        user_id: impl Into<String>,
    ) -> Result<Message<ElinkUserInfoData>> {
        let response = self
            .reqwest_client()
            .get(format!(
                "{}/enlink/api/client/user/findByUserId/{}",
                ROOT_VPN,
                user_id.into()
            ))
            .headers(DEFAULT_HEADERS.clone())
            .send()
            .await?;
        let json = response.text().await?;
        Ok(serde_json::from_str(&json)?)
    }

    async fn webvpn_get_tree_with_service(
        &self,
        user_id: impl Into<String>,
    ) -> Result<Message<ElinkServiceInfoData>> {
        let mut body = HashMap::new();
        body.insert("nameLike", "".to_string());
        body.insert("serviceNameLike", "".to_string());
        body.insert("userId", user_id.into());
        let response = self
            .reqwest_client()
            .post(format!(
                "{}/enlink/api/client/service/group/treeWithService/",
                ROOT_VPN
            ))
            .headers(DEFAULT_HEADERS.clone())
            .header("Referer", format!("{}/enlink/", ROOT_VPN))
            .header("Origin", ROOT_VPN)
            .header("Content-Type", "application/json;charset=utf-8")
            .body(serde_json::to_string(&body)?)
            .send()
            .await?;
        let json = response.text().await?;
        Ok(serde_json::from_str(&json)?)
    }

    async fn webvpn_get_service_by_user(
        &self,
        user_id: impl Into<String>,
    ) -> Result<Message<Vec<ElinkServiceData>>> {
        let mut param = HashMap::new();
        param.insert("name", "");
        let response = self
            .reqwest_client()
            .get(format!(
                "{}/enlink/api/client/service/sucmp/findServiceByUserId/{}",
                ROOT_VPN,
                user_id.into()
            ))
            .headers(DEFAULT_HEADERS.clone())
            .header("Referer", format!("{}/enlink/", ROOT_VPN))
            .header("Origin", ROOT_VPN)
            .query(&param)
            .send()
            .await?;
        let json = response.text().await?;
        Ok(serde_json::from_str(&json)?)
    }

    async fn webvpn_get_visit_service_by_user(
        &self,
        user_id: impl Into<String>,
    ) -> Result<Message<Vec<ElinkServiceData>>> {
        let mut param = HashMap::new();
        param.insert("name", "");
        let response = self
            .reqwest_client()
            .get(format!(
                "{}/enlink/api/client/service/suvisitmp/findVisitServiceByUserId/{}",
                ROOT_VPN,
                user_id.into()
            ))
            .headers(DEFAULT_HEADERS.clone())
            .header("Referer", format!("{}/enlink/", ROOT_VPN))
            .query(&param)
            .send()
            .await?;
        let json = response.text().await?;
        Ok(serde_json::from_str(&json)?)
    }

    /// Client Redirect Policy: [`reqwest::redirect::Policy::none()`]
    async fn webvpn_available(&self) -> bool {
        if let Ok(response) = self.reqwest_client().get(ROOT_SSO_LOGIN).send().await {
            return response.status() == StatusCode::FOUND;
        }
        false
    }

    async fn webvpn_get_proxy_service(
        &self,
        user_id: impl Into<String>,
    ) -> Result<Message<ElinkProxyData>> {
        let response = self
            .reqwest_client()
            .get(format!(
                "{}/enlink/api/client/user/terminal/rules/{}",
                ROOT_VPN,
                user_id.into()
            ))
            .send()
            .await?;
        let json = response.text().await?;
        Ok(serde_json::from_str(&json)?)
    }
}
