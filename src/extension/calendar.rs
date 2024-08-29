use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use icalendar::{Alarm, Calendar, Component, Event, EventLike, Trigger};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, fs::read_to_string, future::Future, io::ErrorKind, path::Path,
    sync::LazyLock,
};
use uuid::Uuid;

use crate::base::typing::TorErr;

pub static EVENT_PROP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("TRANSP", "OPAQUE");
    map.insert("X-APPLE-TRAVEL-ADVISORY-BEHAVIOR", "AUTOMATIC");
    map.insert("SEQUENCE", "0");
    map
});

#[derive(Serialize, Deserialize, Clone)]
pub struct ScheduleElement {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Schedule {
    pub classtime: Vec<ScheduleElement>,
}

impl Default for Schedule {
    fn default() -> Self {
        serde_json::from_str(include_str!("calendar.json")).unwrap()
    }
}

impl Schedule {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        serde_json::from_str(&read_to_string(path).unwrap()).unwrap()
    }

    pub fn from_str(data: &str) -> Self {
        serde_json::from_str(data).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OddOrEven {
    Odd = 0,
    Even = 1,
    Each = 2,
}

#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub name: String,
    pub odd_or_even: OddOrEven,
    pub day: usize,
    pub week: Vec<String>,
    pub classtime: Vec<usize>,
    pub classroom: String,
    pub daylist: Vec<String>,
}

impl ClassInfo {
    pub fn new(
        name: String,
        oe: OddOrEven,
        day: usize,
        week: Vec<String>,
        classtime: Vec<usize>,
        classroom: String,
    ) -> Self {
        Self {
            name,
            odd_or_even: oe,
            day,
            week,
            classtime,
            classroom,
            daylist: vec![],
        }
    }

    pub fn add_classtime(&mut self, value: usize) {
        self.classtime.push(value)
    }

    pub fn add_week(&mut self, value: String) {
        self.week.push(value)
    }

    pub fn merge(&mut self, rhs: &ClassInfo) -> &mut Self {
        rhs.week.iter().for_each(|v| {
            if !self.week.contains(v) {
                self.add_week(v.clone());
            }
        });
        self
    }

    pub fn identify(&self) -> String {
        uuid::Uuid::new_v3(
            &Uuid::NAMESPACE_DNS,
            format!(
                "{}-{}-{}-{}",
                self.name,
                self.odd_or_even.clone() as isize,
                self.day,
                self.classroom
            )
            .as_bytes(),
        )
        .to_string()
    }

    pub fn with_startdate(&mut self, start_date: &str) -> &mut Self {
        let firstdate = NaiveDate::parse_from_str(start_date, "%Y%m%d").unwrap();
        for week in self.week.iter() {
            let v: Vec<i32> = week.split("-").map(|v| v.parse::<i32>().unwrap()).collect();
            let (mut start_week, end_week) = (v[0], v[1]);

            let mut startdate =
                firstdate + Duration::days(((start_week - 1) * 7 + self.day as i32 - 1) as i64);
            let enddate =
                firstdate + Duration::days(((end_week - 1) * 7 + self.day as i32 - 1) as i64);

            loop {
                if self.odd_or_even == OddOrEven::Each
                    || ((self.odd_or_even == OddOrEven::Odd) && (start_week % 2 == 1))
                    || (self.odd_or_even == OddOrEven::Even) && (start_week % 2 == 0)
                {
                    self.daylist.push(startdate.format("%Y%m%d").to_string());
                }
                startdate += Duration::days(7);
                start_week += 1;
                if startdate > enddate {
                    break;
                }
            }
        }
        self
    }
}

pub trait ApplicationCalendarExt {
    fn generate_icalendar_from_classlist(
        &self,
        classlist: Vec<ClassInfo>,
        firstweekdate: String,
        schedule: Schedule,
        reminder: Option<i32>,
    ) -> TorErr<Calendar>;
    fn generate_icalendar(
        &self,
        firstweekdate: String,
        schedule: Schedule,
        reminder: Option<i32>,
    ) -> impl Future<Output = TorErr<Calendar>>;

    fn row_matrix_to_classinfo(&self, row_matrix: Vec<Vec<String>>) -> TorErr<Vec<ClassInfo>> {
        let mut column_matrix: Vec<Vec<String>> = vec![];
        for i in 0..7 {
            let mut tmp: Vec<String> = vec![];
            for v in row_matrix.iter() {
                if let Some(value) = v.get(i) {
                    tmp.push(value.clone())
                } else {
                    return Err(tokio::io::Error::new(
                        ErrorKind::InvalidData,
                        "Parse Classinfo error",
                    ));
                }
            }
            column_matrix.push(tmp.clone());
        }

        let mut course_info: HashMap<String, ClassInfo> = HashMap::new();
        for (day, course_day) in column_matrix.iter().enumerate() {
            for (time, courses_vec) in course_day.iter().enumerate() {
                // Course A / Course B / Course C
                let courses: Vec<String> = courses_vec
                    .split("/")
                    .filter(|v| !v.trim().is_empty())
                    .map(|v| v.trim().to_string())
                    .collect();
                for course in courses {
                    if course == "&nbsp;" || course.is_empty() {
                        continue;
                    }

                    let id = Uuid::new_v3(
                        &Uuid::NAMESPACE_DNS,
                        format!("{}{}", course, day).as_bytes(),
                    )
                    .to_string();

                    let chucks: Vec<String> = course
                        .split(" ")
                        .filter(|c| !c.is_empty())
                        .map(|e| e.trim().to_string())
                        .collect();
                    let name = chucks[0].clone();
                    let place = chucks[1].clone();
                    let oe: String;
                    let week: String;

                    // Name Place Time
                    if chucks.len() == 3 {
                        oe = String::new();
                        week = chucks[2].clone();
                    } else {
                        // Name Place OE Time
                        oe = chucks[2].clone();
                        week = chucks[3].clone();
                    }

                    if !course_info.contains_key(&id) {
                        let info = ClassInfo::new(
                            name,
                            match oe.as_str() {
                                "单" => OddOrEven::Odd,
                                "双" => OddOrEven::Even,
                                _ => OddOrEven::Each,
                            },
                            day + 1,
                            week.split(",")
                                .filter(|e| !e.is_empty())
                                .map(|e| e.to_string())
                                .collect(),
                            vec![time + 1],
                            place,
                        );
                        course_info.insert(id, info);
                    } else {
                        course_info.get_mut(&id).unwrap().add_classtime(time + 1);
                    }
                }
            }
        }

        Ok(course_info
            .values()
            .into_iter()
            .map(|e| e.clone())
            .collect())
    }
}

pub trait CalendarParser {
    /// The Matrix's column is indexed 0~6
    ///
    /// Each Vec<String> is in order.

    fn get_classinfo_week_matrix(&self) -> impl Future<Output = TorErr<Vec<Vec<String>>>>;
}

pub trait TermCalendarParser: CalendarParser {
    fn get_term_classinfo_week_matrix(
        &self,
        term: String,
    ) -> impl Future<Output = TorErr<Vec<Vec<String>>>>;
}

impl<P: CalendarParser> ApplicationCalendarExt for P {
    fn generate_icalendar_from_classlist(
        &self,
        classlist: Vec<ClassInfo>,
        firstweekdate: String,
        schedule: Schedule,
        reminder: Option<i32>,
    ) -> TorErr<Calendar> {
        let mut calendar = Calendar::new();
        calendar.timezone("Asia/Shanghai").name("课程表");
        let mut classlist = classlist;
        classlist.iter_mut().for_each(|e| {
            e.with_startdate(&firstweekdate);
        });

        for info in classlist.iter() {
            let start_time =
                schedule.classtime[info.classtime.first().ok_or_else(|| {
                    tokio::io::Error::new(ErrorKind::InvalidData, "No First data")
                })? - 1]
                    .clone()
                    .start_time;
            let end_time =
                schedule.classtime[info.classtime.last().ok_or_else(|| {
                    tokio::io::Error::new(ErrorKind::InvalidData, "No Last data")
                })? - 1]
                    .clone()
                    .end_time;
            let create_time = Utc::now();
            for day in info.daylist.iter() {
                let uid = format!("{}@gmail.com", Uuid::new_v4());
                let start = NaiveDateTime::parse_from_str(
                    format!("{}{}", day, start_time).as_str(),
                    "%Y%m%d%H%M",
                )
                .unwrap();
                let end = NaiveDateTime::parse_from_str(
                    format!("{}{}", day, end_time).as_str(),
                    "%Y%m%d%H%M",
                )
                .unwrap();

                let mut event = Event::new();

                EVENT_PROP.iter().for_each(|(k, v)| {
                    event.add_property(k.clone(), v.clone());
                });

                event
                    .summary(&info.name)
                    .location(&info.classroom)
                    .timestamp(create_time)
                    .uid(&uid)
                    .starts(start)
                    .ends(end);
                if let Some(reminder) = reminder {
                    event.alarm(Alarm::display(
                        "课前提醒",
                        Trigger::before_start(Duration::minutes(reminder as i64)),
                    ));
                }

                calendar.push(event);
            }
        }

        let mut fweek = NaiveDateTime::new(
            NaiveDate::parse_from_str(&firstweekdate, "%Y%m%d").unwrap(),
            NaiveTime::default(),
        );

        let create_time = Utc::now();
        for wn in 1..=19 {
            let summary = format!("学期第 {} 周", wn);
            let uid = format!("{}@gmail.com", Uuid::new_v4());
            let mut event = Event::new();
            event
                .uid(&uid)
                .summary(&summary)
                .timestamp(create_time)
                .starts(fweek.date())
                .ends(fweek.date() + Duration::days(7));

            EVENT_PROP.iter().for_each(|(k, v)| {
                event.add_property(k, v);
            });

            calendar.push(event.clone());
            fweek += Duration::days(7);
        }

        Ok(calendar)
    }

    async fn generate_icalendar(
        &self,
        firstmonday: String,
        schedule: Schedule,
        reminder: Option<i32>,
    ) -> TorErr<Calendar> {
        let classlist = self.get_classinfo_week_matrix().await?;

        self.generate_icalendar_from_classlist(
            self.row_matrix_to_classinfo(classlist).unwrap(),
            firstmonday,
            schedule,
            reminder,
        )
    }
}
