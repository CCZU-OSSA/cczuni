use crate::{
    base::{
        app::Application,
        client::Client,
        typing::{other_error, TorErr},
    },
    impls::apps::iccard::iccard_type::{
        DormArea, DormBuilding, DormBuildingsData, DormRoomElectricityBillData,
    },
    internals::fields::DEFAULT_HEADERS,
};

pub struct ICCardApplication<C, S> {
    pub client: C,
    pub root: S,
}

impl<C: Client + Clone> Application<C> for ICCardApplication<C, &'static str> {
    async fn from_client(client: C) -> Self {
        Self {
            client,
            root: "http://wxxy.cczu.edu.cn",
        }
    }
}

impl<C: Client + Clone + Send> ICCardApplication<C, &'static str> {
    pub fn endpoint(&self, endpoint: &str) -> String {
        format!("{}/{}", self.root, endpoint)
    }
    pub async fn query_electricity_bill(
        &self,
        area: DormArea<impl Into<String>>,
        building: DormBuilding,
        room: impl Into<String>,
    ) -> TorErr<DormRoomElectricityBillData> {
        let url = self.endpoint("wechat/callinterface/queryElecRoomInfo.html");
        let areaname = area.name.into();
        let areaid = area.id.into();

        let response = self
            .client
            .reqwest_client()
            .post(url)
            .headers(DEFAULT_HEADERS.clone())
            .query(&serde_json::json!({
                "aid": &areaid,
                "account": self.client.account().user,
                "area": serde_json::json!({
                    "area": &areaname,
                    "areaname": &areaname
                }).to_string(),
                "building": serde_json::json!({
                    "building": &building.building,
                    "buildingid": &building.buildingid
                }).to_string(),
                "floor":serde_json::json!({
                    "floorid": "",
                    "floor": ""
                }).to_string(),
                "room": serde_json::json!({
                    "room": "",
                    "roomid": room.into()
                }).to_string(),
            }))
            .send()
            .await
            .map_err(other_error)?;
        Ok(response.json().await.map_err(other_error)?)
    }

    pub async fn query_buildings(
        &self,
        area: DormArea<impl Into<String>>,
    ) -> TorErr<DormBuildingsData> {
        let url = self.endpoint("wechat/callinterface/queryElecBuilding.html");
        let id = area.id.into();
        let name = area.name.into();
        let area = serde_json::json!({
            "area": &name,
            "areaname": &name
        });

        let response = self
            .client
            .reqwest_client()
            .post(url)
            .headers(DEFAULT_HEADERS.clone())
            .query(&serde_json::json!({
                "aid": &id,
                "area": area.to_string(),
                "account": self.client.account().user,
            }))
            .send()
            .await
            .map_err(other_error)?;
        Ok(response.json().await.map_err(other_error)?)
    }
}

#[cfg(test)]
mod ptest {
    use crate::{
        base::app::AppVisitor,
        impls::{
            apps::iccard::{iccard::ICCardApplication, iccard_constants::PRSET_DORMBUILDINGS},
            client::DefaultClient,
        },
    };

    #[tokio::test]
    async fn test() {
        let area = PRSET_DORMBUILDINGS[1].clone();
        let client = DefaultClient::iccard("1");
        let app = client.visit::<ICCardApplication<_, _>>().await;
        let buildings = &app.query_buildings(area.clone()).await.unwrap();
        println!("{:?}", buildings.buildingtab.get(7).unwrap());

        println!(
            "{:?}",
            app.query_electricity_bill(
                area.clone(),
                buildings.buildingtab.get(7).unwrap().clone(),
                "",
            )
            .await
            .unwrap()
        );
    }
}
