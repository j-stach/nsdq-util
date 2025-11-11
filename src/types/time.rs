
pub use chrono::{ NaiveTime, Timelike };
use crate::error::TypeError;

use nom::{
    number::streaming::be_u64,
    bytes::complete::take,
};

/// Parse from OUCH-encoded timestamp to Rust-friendly type.
/// Expects to find 8 bytes (u64).
pub fn parse_ouch_time(
    input: &[u8]
) -> nom::IResult<&[u8], Result<NaiveTime, TypeError>> {

    let (input, nanosec) = be_u64(input)?;

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

/// Parse from OUCH-encoded timestamp to Rust-friendly type.
/// Expects to find 8 bytes (u64).
///
/// It is the same convention used by NASDAQ, 
/// so it should be valid when received from an official data stream.
///
/// # Panics
/// Will panic if the parsed `u64` is greater than 86400e9 (nonsec in a day).
pub fn parse_ouch_time_bold(input: &[u8]) -> nom::IResult<&[u8], NaiveTime> {

    let (input, nanosec) = be_u64(input)?;

    let d = 10u64.pow(9);
    let secs = (nanosec / d) as u32;
    let nano = (nanosec % d) as u32;

    // It is the same convention used by NASDAQ, so it should be valid.
    let time = NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
        .expect("Timestamp is a valid time");

    Ok((input, time))
} 

/// Encode a timestamp to BE u64, representing nanoseconds from midnight.
pub fn encode_ouch_time(time: NaiveTime) -> [u8; 8] {

    let secs = time.num_seconds_from_midnight() as u64;
    let nano = time.nanosecond() as u64;

    let time = secs * 10u64.pow(9) + nano;
    time.to_be_bytes()
}

/// Parse from ITCH-encoded timestamp to Rust-friendly type.
/// Expects to find 6 bytes (not a full u64).
pub fn parse_itch_time(
    input: &[u8]
) -> nom::IResult<&[u8], Result<NaiveTime, TypeError>> {

    let (input, raw) = take(6usize)(input)?;

    let mut buf = [0u8; 8];
    let (_, tail) = buf.split_at_mut(6usize);
    tail.copy_from_slice(&raw);
    let nanosec = u64::from_be_bytes(buf);

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

/// Parse from ITCH-encoded timestamp to Rust-friendly type.
/// Expects to find 6 bytes (not a full u64).
///
/// It is the same convention used by NASDAQ, 
/// so it should be valid when received from an official data stream.
///
/// # Panics
/// Will panic if the parsed `u64` is greater than 86400e9 (nonsec in a day).
pub fn parse_itch_time_bold(input: &[u8]) -> nom::IResult<&[u8], NaiveTime> {

    let (input, raw) = take(6usize)(input)?;

    let mut buf = [0u8; 8];
    let (_, tail) = buf.split_at_mut(6usize);
    tail.copy_from_slice(&raw);
    let nanosec = u64::from_be_bytes(buf);

    let d = 10u64.pow(9);
    let secs = (nanosec / d) as u32;
    let nano = (nanosec % d) as u32;

    // It is the same convention used by NASDAQ, so it should be valid.
    let time = NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
        .expect("Timestamp is a valid time");

    Ok((input, time))
} 

// NOTE: `encode_itch_time` is not necessary.

