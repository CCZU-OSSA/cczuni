use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DormArea<S: Into<String>> {
    pub name: S,
    pub id: S,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DormBuildingsData {
    pub area: DormBuildingsArea,
    pub errmsg: String,
    pub buildingtab: Vec<DormBuilding>,
    pub aid: String,
    pub account: String,
    pub retcode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DormBuildingsArea {
    pub area: String,
    pub areaname: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DormBuilding {
    pub building: String,
    pub buildingid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DormRoomElectricityBillData {
    pub area: DormBuildingsArea,
    pub errmsg: String,
    pub meterflag: String,
    pub bal: String,
    pub building: DormBuilding,
    pub room: DormRoom,
    pub pkgflag: String,
    pub price: String,
    pub pkgtab: Vec<Value>,
    pub floor: DormFloor,
    pub aid: String,
    pub account: String,
    pub retcode: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DormRoom {
    pub roomid: String,
    pub room: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DormFloor {
    pub floorid: String,
    pub floor: String,
}
