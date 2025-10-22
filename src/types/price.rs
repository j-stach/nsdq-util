
/*

use crate::{
    helper::u64_from_be_bytes,
    error::BadElementError,
};

/// Struct for Price that enforces protocol compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price {
    // Maximum accepted value is $199,999.9900
    dollars: u32,
    // Integer to represent the decimal portion of price
    // 9,900 = $0.99
    cents: u16
}

/// Public functions that can be used to create price values.
impl Price {

    /// Checks if custom price is a valid value (i.e., < $199,999.9900).
    /// Helps ensure message encoding is done correctly.
    /// `cents` is actually hundredths of a cent ($0.99 -> 9900 "cents").
    pub fn new(dollars: u32, cents: u16) -> Result<Self, BadElementError> {

        // Ensures price is within limits.
        if dollars > 199_999 || cents > 9999 || 
            (dollars == 199_999 && cents >= 9900) {
            return Err(BadElementError::InvalidValue("Price".to_string()))
        }

        Ok(Price { dollars, cents })
    }

    /// Use $200,000.0000 to flag an order as a market order.
    pub fn market() -> Self {
        Price { 
            dollars: 200_000,
            cents: 0000,
        }
    }

    /// Use $214,748.3647 to flag an order as a market order for a cross.
    pub fn market_cross() -> Self {
        Price {
            dollars: 214_748,
            cents: 3647,
        }
    }

    /// Whole dollars 
    pub fn dollars(&self) -> u32 {  self.dollars }

    /// Remainder in hundredths of a cent ($0.99 -> 9900 "cents")
    pub fn cents(&self) -> u16 { self.cents }
}

impl Price {

    pub(crate) fn encode(&self) -> [u8; 8] {
        // OUCH price has four decimals implied.
        let price: u64 = self.dollars as u64 * 10_000 + self.cents as u64;
        price.to_be_bytes()
    }

    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {

        let price = u64_from_be_bytes(&data)?;
        // OUCH price has four decimals implied.
        Price::new((price / 10_000) as u32, (price % 10_000) as u16)
    }
}


use crate::{
    helper::i32_from_be_bytes,
    error::BadElementError,
};

/// Struct for signed price (in peg offsets) that enforces protocol compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignedPrice {
    negative: bool,
    dollars: u32,
    cents: u16
}

/// Public functions that can be used to create price values.
impl SignedPrice {

    /// Helps ensure message encoding is done correctly.
    /// `cents` is actually hundredths of a cent ($0.99 -> 9900 "cents").
    ///
    /// NOTE: Enforces the same maximum as Price, since any offset greater 
    /// than that is guaranteed to create an invalid price.
    /// Orders can still be rejected if a Peg Offset creates a Price that is 
    /// greater than the maximum allowed.
    pub fn new(
        dollars: u32, 
        cents: u16,
        negative: bool
    ) -> Result<Self, BadElementError> {

        // Ensures price is within limits.
        if dollars > 199_999 || cents > 9999 || 
            (dollars == 199_999 && cents >= 9900) {
            return Err(BadElementError::InvalidValue("SignedPrice".to_string()))
        }

        Ok(SignedPrice { negative, dollars, cents })
    }

    /// Whole dollars 
    pub fn dollars(&self) -> u32 {  self.dollars }

    /// Remainder in hundredths of a cent ($0.99 -> 9900 "cents")
    pub fn cents(&self) -> u16 { self.cents }

    /// Returns true if the signed price is a negative offset.
    pub fn is_negative(&self) -> bool { self.negative }

}

impl SignedPrice {

    pub(crate) fn encode(&self) -> [u8; 4] {
        // OUCH price has four decimals implied.
        let mut price: i32 = self.dollars as i32 * 10_000 + self.cents as i32;
        if self.negative { price = price * -1 }
        price.to_be_bytes()
    }

    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {

        let mut price = i32_from_be_bytes(&data)?;

        // Extract the sign.
        let negative = price < 0;
        price = price.abs();

        // OUCH price has four decimals implied.
        SignedPrice::new(
            (price / 10_000) as u32, 
            (price % 10_000) as u16,
            negative
        )
    }
}
*/
