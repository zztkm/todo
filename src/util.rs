use chrono::prelude::*;

pub fn parse_ymd_hms(date: &str, time: &str) -> DateTime<Utc> {
    let (year, month, day) = {
        let v: Vec<&str> = date.split('-').collect();
        (
            v[0].parse().unwrap(),
            v[1].parse().unwrap(),
            v[2].parse().unwrap(),
        )
    };
    let (hour, minute, second) = {
        let v: Vec<&str> = time.split(':').collect();
        (
            v[0].parse().unwrap(),
            v[1].parse().unwrap(),
            v[2].parse().unwrap(),
        )
    };
    NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, minute, second)
        .unwrap()
        .and_utc()
}
