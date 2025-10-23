
use chrono::NaiveTime;
use crate::error::TypeError;

/// Parse from network-encoded timestamp to Rust-friendly type.
pub fn parse_nanosecs(
    input: &[u8]
) -> nom::IResult<&[u8], Result<NaiveTime, TypeError>> {

    let (input, nanosec) = nom::number::streaming::be_u64(input)?;

    let d = 10u64.pow(9);
    let secs = (nanosec / d) as u32;
    let nano = (nanosec % d) as u32;

    // It is the same convention used by NASDAQ, so it should be valid.
    // But... just in case, we check.
    let time = if let Some(naive) = 
        NaiveTime::from_num_seconds_from_midnight_opt(secs, nano) { 
            Ok(naive) 
        } else {
            Err(TypeError::InvalidTime(nanosec))
        };

    Ok((input, time))
} 

/// Parse from network-encoded timestamp to Rust-friendly type.
/// It is the same convention used by NASDAQ, 
/// so it should be valid when received from an official data stream.
///
/// # Panics
/// Will panic if the parsed `u64` greater than 86400e9 (nonsec in a day).
pub fn parse_nanosecs_bold(input: &[u8]) -> nom::IResult<&[u8], NaiveTime> {

    let (input, nanosec) = nom::number::streaming::be_u64(input)?;

    let d = 10u64.pow(9);
    let secs = (nanosec / d) as u32;
    let nano = (nanosec % d) as u32;

    // It is the same convention used by NASDAQ, so it should be valid.
    let time = NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
        .expect("Timestamp is a valid time");

    Ok((input, time))
} 

