use crate::impls::apps::iccard::iccard_type::DormArea;

// {"area": "西太湖校区", "areaname": "西太湖校区", "aid": "0030000000002501"},
// {"area": "武进校区", "areaname": "武进校区", "aid": "0030000000002502"},
// {
//     "area": "西太湖校区1-7,10-11",
//     "areaname": "西太湖校区1-7,10-11",
//     "aid": "0030000000002503",
// },
pub const PRSET_DORMBUILDINGS: [DormArea<&'static str>; 3] = [
    DormArea {
        name: "西太湖校区",
        id: "0030000000002501",
    },
    DormArea {
        name: "武进校区",
        id: "0030000000002502",
    },
    DormArea {
        name: "西太湖校区1-7,10-11",
        id: "0030000000002503",
    },
];
