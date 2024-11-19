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

pub fn parse_start_date_time(date: Option<String>, time: Option<String>) -> DateTime<Utc> {
    let current_date_time = Utc::now();
    let start_date = if date.is_some() && time.is_some() {
        let date = date.as_ref().unwrap();
        let time = time.as_ref().unwrap();
        parse_ymd_hms(date, time)
    } else if date.is_some() && time.is_none() {
        let date = date.as_ref().unwrap();
        let time = "00:00:00";
        parse_ymd_hms(date, time)
    } else if date.is_none() && time.is_some() {
        let date = &current_date_time.format("%Y-%m-%d").to_string();
        let time = time.as_ref().unwrap();
        parse_ymd_hms(date, time)
    } else {
        current_date_time
    };
    start_date
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_start_date_time() {
        let date = Some("2024-12-31".to_string());
        let time = Some("12:34:56".to_string());
        let dt = parse_start_date_time(date, time);

        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 31);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 34);
        assert_eq!(dt.second(), 56);
    }
}
