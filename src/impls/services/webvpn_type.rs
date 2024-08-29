use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Message<T> {
    pub code: String,
    pub messages: String,
    pub data: T,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkProxyData {
    pub token: String,
    pub server: String,
    pub tunnel_free_interval: i32,
    pub tunnel_free_status: bool,
    pub spa_status: bool,
    pub fwd_status: bool,
    pub spa_port: String,
    pub admin_port: String,
    // TODO May fill other data future
    pub gateway_list: Vec<ElinkProxyGatewayList>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ElinkProxyGatewayList {
    pub dns: String,
    // ...
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkGroupInfo {
    pub name: String,
    pub id: String,
    pub description: String,
    pub creator: String,
    #[serde(rename = "createTime")]
    pub createtime: String, // 格式: yyyy-MM-dd HH:mm:ss
    pub updator: String,
    #[serde(rename = "authTypeId")]
    pub auth_type_id: String,
    #[serde(rename = "updateTime")]
    pub updatetime: String, // 格式: yyyy-MM-dd HH:mm:ss
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkUserInfoData {
    pub username: String,
    pub name: String,
    pub id: String,
    pub email: String,
    pub mobile: String,
    #[serde(rename = "userState")]
    pub user_state: String,
    #[serde(rename = "lastLoginTime")]
    pub lastlogintime: String, // 格式: yyyy-MM-dd HH:mm:ss
    #[serde(rename = "userGroups")]
    pub user_groups: Vec<ElinkGroupInfo>,
    pub creator: String,
    #[serde(rename = "createTime")]
    pub createtime: String, // 格式: yyyy-MM-dd HH:mm:ss
    pub updator: String,
    #[serde(rename = "updateTime")]
    pub updatetime: String, // 格式: yyyy-MM-dd HH:mm:ss
    #[serde(rename = "dingNickName")]
    pub ding_nick_name: String,
    #[serde(rename = "qyWeChatUserId")]
    pub qy_we_chat_user_id: String,
    #[serde(rename = "weChatNickName")]
    pub we_chat_nick_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkServiceInfoData {
    pub title: String,
    pub key: String,
    pub children: Option<Vec<ElinkServiceInfoData>>,
    #[serde(rename = "serviceList")]
    pub service_list: Option<Vec<ElinkServiceData>>,
    #[serde(rename = "serviceAllList")]
    pub service_all_list: Option<Vec<ElinkServiceData>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkServiceGatewayData {
    pub id: String,
    pub name: String,
    #[serde(rename = "uniqueNo")]
    pub unique_no: String,
    pub server: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_of: String,
    #[serde(rename = "adminAddr")]
    pub admin_addr: String,
    #[serde(rename = "nginxPort")]
    pub nginx_port: String,
    #[serde(rename = "connectState")]
    pub connect_state: String,
    #[serde(rename = "publicServer")]
    pub public_server: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElinkServiceData {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub server: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_of: String,
    #[serde(rename = "urlPlus")]
    pub url_plus: String,
    #[serde(rename = "hostMd5")]
    pub host_md5: String,
    #[serde(rename = "gatewayVo")]
    pub gateway_vo: Option<ElinkServiceGatewayData>,
}
