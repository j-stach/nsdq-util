
use crate::error::TypeError;

/// Prices are integer fields, supplied with an associated precision. 
/// When converted to a decimal format, prices are in fixed point format, 
/// where `N` defines the number of decimal places. 
/// For example, Price<I, 4> has an implied 4 decimal places. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price<I, const N: u8> {
    /// Dollar and cents together, with the decimal marked by `precision`.
    val: I,
}

impl<I: Copy + PartialOrd + std::fmt::Display, const N: u8> Price<I, N> 
    where i64: From<I> 
{

    /// The maximum price one would need to create is $199,999.9900,
    /// which is the maximum order price for OUCH.
    ///```
    /// use nsdq_util::types::Price;
    ///
    /// let price = Price<u32, 4>::new(35000u32).unwrap();
    /// let (dollars, cents) = price.parts();
    /// assert_eq!(dollars, 3u32);
    /// assert_eq!(cents, 5000u32);
    ///```
    pub fn new(val: I) -> Result<Self, TypeError> {
        // Should always be in range
        if i64::from(val) <= 199_999_9900i64 {
            Ok(Self { val })
        } else {
            Err(TypeError::InvalidPrice(format!("{}", val)))
        }
    }

    /// Copy the price with dollars and cents together.
    pub fn val(&self) -> I { self.val }
}

impl<const N: u8> Price<u32, N> {

    /// Returns whole dollars, remainder (cents)
    pub fn parts(&self) -> (u32, u32) {
        let denom = 10_u32.pow(N as u32);
        let dollars = self.val / denom;
        let cents = self.val % denom;
        (dollars, cents)
    }

    /// Encode price as big-endian bytes.
    pub fn encode(&self) -> [u8; 4] {
        self.val.to_be_bytes()
    }

    /// Parse price from 4 bytes.
    pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        let (input, val) = nom::number::streaming::be_u32(input)?;
        Ok((input, Self { val }))
    }
}

impl<const N: u8> Price<i32, N> {

    /// Returns whole dollars, remainder (cents)
    pub fn parts(&self) -> (i32, u32) {
        let denom = 10_i32.pow(N as u32);
        let dollars = self.val / denom;
        let cents = self.val % denom;
        (dollars, cents.abs() as u32)
    }

    /// Encode price as big-endian bytes.
    pub fn encode(&self) -> [u8; 4] {
        self.val.to_be_bytes()
    }

    /// Parse signed price from 4 bytes.
    pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        let (input, val) = nom::number::streaming::be_i32(input)?;
        Ok((input, Self { val }))
    }
}

impl<const N: u8> Price<u64, N> {

    /// Returns whole dollars, remainder (cents)
    pub fn parts(&self) -> (u64, u64) {
        let denom = 10_u64.pow(N as u32);
        let dollars = self.val / denom;
        let cents = self.val % denom;
        (dollars, cents)
    }

    /// Use $200,000.0000 to flag an OUCH order as a market order.
    pub fn market() -> Self {
        Price { val: 200_000_0000u64 }
    }

    /// Use $214,748.3647 to flag an OUCH cross order as a market order.
    pub fn market_cross() -> Self {
        Price { val: 214_748_3647u64 }
    }

    /// Encode price as big-endian bytes.
    pub fn encode(&self) -> [u8; 8] {
        self.val.to_be_bytes()
    }

    /// Parse price from 8 bytes.
    pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        let (input, val) = nom::number::streaming::be_u64(input)?;
        Ok((input, Self { val }))
    }
}

