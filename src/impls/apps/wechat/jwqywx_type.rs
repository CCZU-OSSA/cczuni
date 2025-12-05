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
    #[serde(rename = "yhid")]
    pub id: String,
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
    //  #[serde(rename = "pscj")]
    //  pub usual_grade: f32,
    //  #[serde(rename = "qzcj")]
    //  pub mid_exam_grade: f32,
    //  #[serde(rename = "qmcj")]
    //  pub end_exam_grade: f32,
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
    // #[serde(rename = "nj")]
    // pub grade: String,
    #[serde(rename = "bh")]
    pub class_id: String,
    #[serde(rename = "bj")]
    pub class_name: String,
    #[serde(rename = "xh")]
    pub student_id: String,
    #[serde(rename = "xm")]
    pub student_name: String,
    #[serde(rename = "xb")]
    pub student_gender: String,
    #[serde(rename = "xjqk")]
    pub student_status: String,
    #[serde(rename = "csny")]
    pub student_birthday: String,
    #[serde(rename = "xsid")]
    pub student_xid: String,
    #[serde(rename = "pjxfjd")]
    pub grade_points: f32,
    // #[serde(rename = "pm")]
    // pub rank: String,
    // #[serde(rename = "zypm")]
    // pub major_rank: String,
    // #[serde(rename = "zxfjd")]
    // pub total_grade_points: String,
    // #[serde(rename = "zxf")]
    // pub total_credits: String,
    // #[serde(rename = "pjcjxf")]
    // pub average_credits: String,
    // #[serde(rename = "pjxfjd")]
    // pub average_grade_points: String,
    // #[serde(rename = "pjcj")]
    // pub average_grade: String,
}

#[derive(Debug, Deserialize)]
pub struct Term {
    #[serde(rename = "xq")]
    pub term: String,
}

#[derive(Debug, Deserialize)]
pub struct Exam {
    #[serde(rename = "lb")]
    pub category: String,
    #[serde(rename = "xq")]
    pub term: String,
    #[serde(rename = "xsbh")]
    pub student_number: String,
    #[serde(rename = "xsbj")]
    pub class_full_name: String,
    #[serde(rename = "xh")]
    pub student_id: String,
    #[serde(rename = "xm")]
    pub student_name: String,
    #[serde(rename = "BH")]
    pub class_code: String,
    #[serde(rename = "kcdm")]
    pub course_code: String,
    #[serde(rename = "kch")]
    pub course_serial: String,
    #[serde(rename = "kcmc")]
    pub course_name: String,
    #[serde(rename = "xklb")]
    pub course_category: String,
    #[serde(rename = "yx")]
    pub active_flag: i32,
    pub id: i32,
    #[serde(rename = "ksz")]
    pub start_week: Option<i32>,
    #[serde(rename = "zc")]
    pub week: Option<i32>,
    #[serde(rename = "jc1")]
    pub lesson_start: Option<i32>,
    #[serde(rename = "jc2")]
    pub lesson_end: Option<i32>,
    #[serde(rename = "jse")]
    pub classroom: Option<String>,
    #[serde(rename = "sj")]
    pub time_range: Option<String>,
    #[serde(rename = "jkjs1")]
    pub invigilator_one: Option<String>,
    #[serde(rename = "jkjs2")]
    pub invigilator_two: Option<String>,
    #[serde(rename = "bz")]
    pub remark: Option<String>,
    #[serde(rename = "bmmc")]
    pub campus: String,
    #[serde(rename = "kssj")]
    pub exam_time: Option<String>,
    #[serde(rename = "bj")]
    pub class_name: String,
    #[serde(rename = "sjxx")]
    pub schedule_details: Option<String>,
    #[serde(rename = "jseid")]
    pub classroom_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct EvaluatableClass {
    #[serde(rename = "bh")]
    pub class_id: String,
    #[serde(rename = "kcdm")]
    pub course_code: String,
    #[serde(rename = "kcmc")]
    pub course_name: String,
    #[serde(rename = "kch")]
    pub course_serial: String,
    #[serde(rename = "lbdh")]
    pub category_code: String,
    #[serde(rename = "jsdm")]
    pub teacher_code: String,
    #[serde(rename = "jsmc")]
    pub teacher_name: String,
    #[serde(rename = "pjqk")]
    pub evaluation_status: Option<String>,
    #[serde(rename = "pjid")]
    pub evaluation_id: i32,
    #[serde(rename = "jsid")]
    pub teacher_id: String,
}

#[cfg(feature = "calendar")]
pub mod calendar {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde_json::Value;

    use crate::extension::calendar::RawCourse;

    /// Fuck the stupid noob programmer 😅
    ///
    /// TBH, I really don't know how to put 100+ field in a struct.
    /// So I select some important field.
    ///
    /// If you want the other fields, just create a new struct yourself.
    ///
    /// There is no need to use a proc-macros here, IMO, enumerate each field is better.
    #[derive(Debug, Deserialize, Clone)]
    pub struct SerdeRowCourses {
        #[serde(flatten)]
        pub fields: HashMap<String, Value>,
    }

    impl Into<Vec<RawCourse>> for SerdeRowCourses {
        fn into(self) -> Vec<RawCourse> {
            let courses = (1..=7).map(|index| {
                let value = self.fields.get(&format!("kc{index}"));
                if let Some(Value::String(course)) = value {
                    return course.clone();
                }

                String::new()
            });

            let mut teachers = HashMap::new();

            for index in 1..=20 {
                let name = self.fields.get(&format!("kcmc{index}"));
                if let Some(Value::String(name)) = name {
                    if let Some(Value::String(teacher)) = self.fields.get(&format!("skjs{index}")) {
                        teachers.insert(name.clone(), teacher.clone());
                    }
                }
            }
            courses
                .map(|course| {
                    let teacher = course
                        .split("/")
                        .map(|single| {
                            teachers
                                .get(
                                    single
                                        .split(" ")
                                        .collect::<Vec<&str>>()
                                        .first()
                                        .cloned()
                                        .unwrap_or(""),
                                )
                                .cloned()
                                .unwrap_or(String::new())
                        })
                        .reduce(|a, b| {
                            if b.is_empty() {
                                return a;
                            }
                            return format!("{},/{}", a, b);
                        })
                        .unwrap_or(String::new());

                    RawCourse { course, teacher }
                })
                .collect()
        }
    }
}
#[cfg(feature = "calendar")]
pub use calendar::SerdeRowCourses;
