
/// Parse from network-encoded timestamp to Rust-friendly type.
pub fn parse_nanosecs(input: &[u8]) -> nom::IResult<&[u8], chrono::NaiveTime> {

    let (input, nanosec) = nom::number::streaming::be_u64(input)?;

    let d = 10u64.pow(9);
    let secs = (nanosec / d) as u32;
    let nano = (nanosec % d) as u32;

    // TODO: TBD: Is this safe to expect?
    // It is the same convention used by OUCH, so it should not be invalid.
    let time = chrono::NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
        .expect("Timestamp is a valid time");

    Ok((input, time))
} 

// TODO Error type that handles invalid time?
