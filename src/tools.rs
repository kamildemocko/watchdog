use chrono::{DateTime, Utc};

pub fn conv_unix_datetime(dt: u64) -> DateTime<Utc> {
    DateTime::from_timestamp(dt as i64, 0).unwrap()
}
