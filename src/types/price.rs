
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

impl<I: num_traits::PrimInt + std::fmt::Display, const N: u8> Price<I, N> {

    /// The maximum price one would need to create is $199,999.9900,
    /// which is the maximum order price for OUCH.
    /// NOTE: This function will always fail for N < 2 
    /// (cannot create whole dollars without a fractional component).
    ///```
    /// use nsdq_util::Price;
    ///
    /// let price = Price::<u32, 4>::new(35000u32).unwrap();
    /// let (dollars, cents) = price.parts();
    /// assert_eq!(dollars, 3u32);
    /// assert_eq!(cents, 5000u32);
    ///```
    pub fn new(val: I) -> Result<Self, TypeError> {

        if N < 2 {
            return Err(TypeError::InvalidPrice(String::from("N")))
        }

        // Should always be in range
        // Subtract 2 to account for $0.99
        let mag = I::from(10).unwrap().pow((N as u32) - 2);
        let limit = I::from(199_999_99).unwrap() * mag;

        if val <= limit {
            Ok(Self { val })
        } else {
            Err(TypeError::InvalidPrice(format!("{}", val)))
        }
    }

    /// Copy the price with dollars and cents together.
    pub fn val(&self) -> I { self.val }
}


/// `Price<u32, 4>` is used for added orders in ITCH.
///```
/// use nsdq_util::Price;
///
/// let price = Price::<u32, 4>::new(35000u32).unwrap();
/// let (dollars, cents) = price.parts();
/// assert_eq!(dollars, 3u32);
/// assert_eq!(cents, 5000u32);
///
/// let bytes = 35000u32.to_be_bytes();
/// let (_, price2) = Price::<u32, 4>::parse(&bytes).unwrap();
/// assert_eq!(price, price2);
/// assert_eq!(price.encode(), bytes);
///```
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


/// `Price<i32, 4>` is used for setting peg offsets in OUCH.
///```
/// use nsdq_util::Price;
///
/// let price = Price::<i32, 4>::new(-35000i32).unwrap();
/// let (dollars, cents) = price.parts();
/// assert_eq!(dollars, -3i32);
/// assert_eq!(cents, 5000u32);
///
/// let bytes = (-35000i32).to_be_bytes();
/// let (_, price2) = Price::<i32, 4>::parse(&bytes).unwrap();
/// assert_eq!(price, price2);
/// assert_eq!(price.encode(), bytes);
///```
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


/// `Price<u64, 4>` is used for order entry in OUCH,
/// and `Price<u64, 8>` is used for the MWCB levels in ITCH.
///```
/// use nsdq_util::Price;
///
/// // Price<u64, 4>
///
/// let price = Price::<u64, 4>::new(3_5000u64).unwrap();
/// let (dollars, cents) = price.parts();
/// assert_eq!(dollars, 3u64);
/// assert_eq!(cents, 5000u64);
///
/// let bytes = 3_5000u64.to_be_bytes();
/// let (_, price2) = Price::<u64, 4>::parse(&bytes).unwrap();
/// assert_eq!(price, price2);
/// assert_eq!(price.encode(), bytes);
///
/// // Price<u64, 8>
///
/// let price = Price::<u64, 8>::new(3_5000_0000u64).unwrap();
/// let (dollars, cents) = price.parts();
/// assert_eq!(dollars, 3u64);
/// assert_eq!(cents, 5000_0000u64);
///
/// let bytes = 3_5000_0000u64.to_be_bytes();
/// let (_, price2) = Price::<u64, 8>::parse(&bytes).unwrap();
/// assert_eq!(price, price2);
/// assert_eq!(price.encode(), bytes);
///```
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

