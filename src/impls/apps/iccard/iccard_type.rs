use serde::{Deserialize, Serialize};

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
