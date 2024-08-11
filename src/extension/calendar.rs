use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use icalendar::{Alarm, Calendar, Component, Event, EventLike, Trigger};
use std::{collections::HashMap, fs::read_to_string, future::Future, path::Path, sync::LazyLock};
use uuid::Uuid;

use crate::base::typing::TorErr;

pub static EVENT_PROP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("TRANSP", "OPAQUE");
    map.insert("X-APPLE-TRAVEL-ADVISORY-BEHAVIOR", "AUTOMATIC");
    map.insert("SEQUENCE", "0");
    map
});

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ScheduleElement {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
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

#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub name: String,
    pub oe: usize,
    pub day: usize,
    pub week: Vec<String>,
    pub classtime: Vec<usize>,
    pub classroom: String,
    pub daylist: Vec<String>,
}

impl ClassInfo {
    pub fn new(
        name: String,
        oe: usize,
        day: usize,
        week: Vec<String>,
        classtime: Vec<usize>,
        classroom: String,
    ) -> Self {
        Self {
            name,
            oe,
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
            format!("{}-{}-{}-{}", self.name, self.oe, self.day, self.classroom).as_bytes(),
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
                if self.oe == 3
                    || ((self.oe == 1) && (start_week % 2 == 1))
                    || (self.oe == 2) && (start_week % 2 == 0)
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
    ) -> Option<Calendar>;
    fn generate_icalendar(
        &self,
        firstweekdate: String,
        schedule: Schedule,
        reminder: Option<i32>,
    ) -> impl Future<Output = Option<Calendar>>;
}

pub trait CalendarParser {
    fn get_classinfo_vec(&self) -> impl Future<Output = TorErr<Vec<ClassInfo>>>;
}

impl<P: CalendarParser> ApplicationCalendarExt for P {
    fn generate_icalendar_from_classlist(
        &self,
        classlist: Vec<ClassInfo>,
        firstweekdate: String,
        schedule: Schedule,
        reminder: Option<i32>,
    ) -> Option<Calendar> {
        let mut calendar = Calendar::new();
        calendar.timezone("Asia/Shanghai").name("课程表");
        let mut classlist = classlist;
        classlist.iter_mut().for_each(|e| {
            e.with_startdate(&firstweekdate);
        });

        for info in classlist.iter() {
            let start_time = schedule.classtime[info.classtime.first()? - 1]
                .clone()
                .start_time;
            let end_time = schedule.classtime[info.classtime.last()? - 1]
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
                    event.add_property(k, v);
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

        // week

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

        Some(calendar)
    }

    async fn generate_icalendar(
        &self,
        firstweekdate: String,
        schedule: Schedule,
        reminder: Option<i32>,
    ) -> Option<Calendar> {
        if let Ok(classlist) = self.get_classinfo_vec().await {
            self.generate_icalendar_from_classlist(classlist, firstweekdate, schedule, reminder)
        } else {
            None
        }
    }
}
