use std::{collections::HashMap, future::Future, io::ErrorKind};

use reqwest::StatusCode;

use crate::{
    base::{client::Client, typing::TorErr},
    internals::fields::{DEFAULT_HEADERS, ROOT_SSO_LOGIN, ROOT_VPN},
};

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
    ) -> impl Future<Output = TorErr<Message<ElinkUserInfoData>>>;
    fn webvpn_get_tree_with_service(
        &self,
        user_id: impl Into<String>,
    ) -> impl Future<Output = TorErr<Message<ElinkServiceInfoData>>>;
    fn webvpn_get_service_by_user(
        &self,
        user_id: impl Into<String>,
    ) -> impl Future<Output = TorErr<Message<Vec<ElinkServiceData>>>>;
    fn webvpn_get_visit_service_by_user(
        &self,
        user_id: impl Into<String>,
    ) -> impl Future<Output = TorErr<Message<Vec<ElinkServiceData>>>>;
    fn webvpn_get_proxy_service(
        &self,
        user_id: impl Into<String>,
    ) -> impl Future<Output = TorErr<Message<ElinkProxyData>>>;
}

impl<C: Client> WebVPNService for C {
    async fn webvpn_get_user_info(
        &self,
        user_id: impl Into<String>,
    ) -> TorErr<Message<ElinkUserInfoData>> {
        if let Ok(response) = self
            .reqwest_client()
            .get(format!(
                "{}/enlink/api/client/user/findByUserId/{}",
                ROOT_VPN,
                user_id.into()
            ))
            .headers(DEFAULT_HEADERS.clone())
            .send()
            .await
        {
            if let Ok(json) = response.text().await {
                return Ok(serde_json::from_str(json.as_str())?);
            }
        }
        Err(tokio::io::Error::new(
            ErrorKind::Other,
            "Get User Info failed",
        ))
    }

    async fn webvpn_get_tree_with_service(
        &self,
        user_id: impl Into<String>,
    ) -> TorErr<Message<ElinkServiceInfoData>> {
        let mut body = HashMap::new();
        body.insert("nameLike", "".to_string());
        body.insert("serviceNameLike", "".to_string());
        body.insert("userId", user_id.into());
        if let Ok(response) = self
            .reqwest_client()
            .post(format!(
                "{}/enlink/api/client/service/group/treeWithService/",
                ROOT_VPN
            ))
            .headers(DEFAULT_HEADERS.clone())
            .header("Referer", format!("{}/enlink/", ROOT_VPN))
            .header("Origin", ROOT_VPN)
            .header("Content-Type", "application/json;charset=utf-8")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await
        {
            if let Ok(json) = response.text().await {
                return Ok(serde_json::from_str(json.as_str())?);
            }
        }
        Err(tokio::io::Error::new(
            ErrorKind::Other,
            "Get Tree Service failed",
        ))
    }

    async fn webvpn_get_service_by_user(
        &self,
        user_id: impl Into<String>,
    ) -> TorErr<Message<Vec<ElinkServiceData>>> {
        let mut param = HashMap::new();
        param.insert("name", "");
        if let Ok(response) = self
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
            .await
        {
            if let Ok(json) = response.text().await {
                return Ok(serde_json::from_str(json.as_str())?);
            }
        }
        Err(tokio::io::Error::new(
            ErrorKind::Other,
            "Get User Service failed",
        ))
    }

    async fn webvpn_get_visit_service_by_user(
        &self,
        user_id: impl Into<String>,
    ) -> TorErr<Message<Vec<ElinkServiceData>>> {
        let mut param = HashMap::new();
        param.insert("name", "");
        if let Ok(response) = self
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
            .await
        {
            if let Ok(json) = response.text().await {
                return Ok(serde_json::from_str(json.as_str())?);
            }
        }
        Err(tokio::io::Error::new(
            ErrorKind::Other,
            "Get User Visit Service failed",
        ))
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
    ) -> TorErr<Message<ElinkProxyData>> {
        if let Ok(response) = self
            .reqwest_client()
            .get(format!(
                "{}/enlink/api/client/user/terminal/rules/{}",
                ROOT_VPN,
                user_id.into()
            ))
            .send()
            .await
        {
            if let Ok(json) = response.text().await {
                return Ok(serde_json::from_str(json.as_str())?);
            }
        }
        Err(tokio::io::Error::new(
            ErrorKind::Other,
            "Get Proxy Service failed",
        ))
    }
}
