pub mod outer {
    use chrono::Utc;
    use chrono::DateTime;

    pub fn to_timestamp_millis(datetime: &str) -> i64 {
        let datetime_utc = DateTime::parse_from_rfc3339(datetime)
            .expect("Failed to parse datetime")
            .with_timezone(&Utc);
        datetime_utc.timestamp_millis()
    }

}


