
use chrono::NaiveTime;

// Convert from encoded timestamp to Rust-friendly type.
// TODO parse
pub fn nanosec_from_midnight(time: u64) -> NaiveTime {

    let d = 10u64.pow(9);
    let secs = (time / d) as u32;
    let nano = (time % d) as u32;

    // TODO: TBD: Is this safe to expect?
    // It is the same convention used by OUCH, so it should not be invalid.
    NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
        .expect("Timestamp is a valid time")
} 

