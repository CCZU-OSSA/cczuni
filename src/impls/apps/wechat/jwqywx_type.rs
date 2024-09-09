use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Message<T> {
    pub status: i32,
    pub message: Vec<T>,
    pub token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LoginUserData {
    #[serde(rename = "yhdm")]
    pub userid: String,
    #[serde(rename = "yhmc")]
    pub username: String,
    #[serde(rename = "yhsf")]
    pub userident: String,
    #[serde(rename = "xq")]
    pub term: String,
    #[serde(rename = "dqz")]
    pub current_value: i32,
    #[serde(rename = "zc")]
    pub position: i32,
    #[serde(rename = "gh")]
    pub employee_number: String,
    pub smscode: String,
    #[serde(rename = "xb")]
    pub gender: String,
    #[serde(rename = "yhqx")]
    pub permission: String,
}

#[derive(Deserialize, Debug)]
pub struct CourseGrade {
    #[serde(rename = "bh")]
    pub class_id: String,
    #[serde(rename = "bj")]
    pub class_name: String,
    #[serde(rename = "xh")]
    pub student_id: String,
    #[serde(rename = "xm")]
    pub student_name: String,
    #[serde(rename = "kcdm")]
    pub course_id: String,
    #[serde(rename = "kcmc")]
    pub course_name: String,
    #[serde(rename = "xq")]
    pub term: i32,
    #[serde(rename = "kclb")]
    pub course_type: String,
    #[serde(rename = "lbmc")]
    pub course_type_name: String,
    #[serde(rename = "xs")]
    pub course_hours: i32,
    #[serde(rename = "xf")]
    pub course_credits: f32,
    #[serde(rename = "jsmc")]
    pub teacher_name: String,
    #[serde(rename = "ksxzm")]
    pub is_exam_type: i32,
    #[serde(rename = "ksxz")]
    pub exam_type: String,
    #[serde(rename = "pscj")]
    pub usual_grade: f32,
    #[serde(rename = "qzcj")]
    pub mid_exam_grade: f32,
    #[serde(rename = "qmcj")]
    pub end_exam_grade: f32,
    #[serde(rename = "kscj")]
    pub exam_grade: String,
    #[serde(rename = "idn")]
    pub ident: i32,
    #[serde(rename = "cj")]
    pub grade: f32,
    #[serde(rename = "xfjd")]
    pub grade_points: f32,
}

#[derive(Deserialize, Debug)]
pub struct StudentPoint {
    #[serde(rename = "nj")]
    pub grade: String,
    #[serde(rename = "bh")]
    pub class_id: String,
    #[serde(rename = "bj")]
    pub class_name: String,
    #[serde(rename = "xh")]
    pub student_id: String,
    #[serde(rename = "xm")]
    pub student_name: String,
    #[serde(rename = "xfjd")]
    pub grade_points: String,
    #[serde(rename = "pm")]
    pub rank: String,
    #[serde(rename = "zypm")]
    pub major_rank: String,
    #[serde(rename = "zxfjd")]
    pub total_grade_points: String,
    #[serde(rename = "zxf")]
    pub total_credits: String,
    #[serde(rename = "pjcjfx")]
    pub average_credits: String,
    #[serde(rename = "pjxfjd")]
    pub average_grade_points: String,
    #[serde(rename = "pjcj")]
    pub average_grade: String,
}

#[derive(Debug, Deserialize)]
pub struct Term {
    #[serde(rename = "xq")]
    pub term: String,
}

#[cfg(feature = "calendar")]
pub mod calendar {
    use serde::Deserialize;

    use crate::extension::calendar::RawCourse;

    /// Fuck the stupid noob programmer 😅
    ///
    /// TBH, I really don't know how to put 100+ field in a struct.
    /// So I select some important field.
    ///
    /// If you want the other fields, just create a new struct yourself.
    ///
    /// There is no need to use a macros here, IMO, enumerate each field is better.
    #[derive(Debug, Deserialize, Clone)]
    pub struct SerdeRowCourses {
        #[serde(rename = "xq")]
        pub term: String,
        #[serde(rename = "kc1")]
        pub course_0: String,
        #[serde(rename = "skjs1")]
        pub tearcher_0: String,
        #[serde(rename = "kc2")]
        pub course_1: String,
        #[serde(rename = "skjs2")]
        pub tearcher_1: String,
        #[serde(rename = "kc3")]
        pub course_2: String,
        #[serde(rename = "skjs3")]
        pub tearcher_2: String,
        #[serde(rename = "kc4")]
        pub course_3: String,
        #[serde(rename = "skjs4")]
        pub tearcher_3: String,
        #[serde(rename = "kc5")]
        pub course_4: String,
        #[serde(rename = "skjs5")]
        pub tearcher_4: String,
        #[serde(rename = "kc6")]
        pub course_5: String,
        #[serde(rename = "skjs6")]
        pub tearcher_5: String,
        #[serde(rename = "kc7")]
        pub course_6: String,
        #[serde(rename = "skjs7")]
        pub tearcher_6: String,
    }

    impl Into<Vec<RawCourse>> for SerdeRowCourses {
        fn into(self) -> Vec<RawCourse> {
            vec![
                RawCourse::new(self.course_0, self.tearcher_0),
                RawCourse::new(self.course_1, self.tearcher_1),
                RawCourse::new(self.course_2, self.tearcher_2),
                RawCourse::new(self.course_3, self.tearcher_3),
                RawCourse::new(self.course_4, self.tearcher_4),
                RawCourse::new(self.course_5, self.tearcher_5),
                RawCourse::new(self.course_6, self.tearcher_6),
            ]
        }
    }
}
#[cfg(feature = "calendar")]
pub use calendar::SerdeRowCourses;
