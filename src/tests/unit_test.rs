use crate::utils;
use chrono;
#[test]
fn test_native_dt_to_timestamp() {

    let test_timestamp: i64 = 1703228154;
    let tests_nsecs: u32 = 0;

    let naive_dt = chrono::NaiveDateTime::from_timestamp_opt(test_timestamp, tests_nsecs);
    let got_timestamp = utils::native_dt_to_timestamp(naive_dt);

    assert_eq!(test_timestamp, got_timestamp.unwrap().seconds)
}