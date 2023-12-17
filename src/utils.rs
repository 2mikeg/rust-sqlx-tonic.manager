use prost_types::Timestamp;
use chrono::NaiveDateTime;

pub fn native_dt_to_timestamp(naive_dt: Option<NaiveDateTime>) -> Option<Timestamp> {
    naive_dt.map(|naive_dt| {
        let timestamp = Timestamp {
            seconds: naive_dt.timestamp(),
            nanos: naive_dt.timestamp_subsec_nanos() as i32,
        };
        timestamp
    })
}