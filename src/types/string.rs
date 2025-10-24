
/// Define a fixed-length string type.
/// ```
/// use nsdq_util::define_str;
///
/// define_str!(MyStr [4usize] "String with a fixed length of 4 characters.");
///
/// let bytes = b"XXXX";
/// let (_, mystr) = MyStr::parse(bytes).unwrap();
///
/// assert_eq!(mystr.encode(), *bytes);
/// assert_eq!(mystr.to_str(), "XXXX");
///
/// assert_eq!(format!("{}", mystr), String::from("XXXX"));
/// ```
/// NOTE: `new` or `from_str` functions are not included in this macro,
/// in case there are special constraints on the character types.
#[macro_export] macro_rules! define_str {
    ($name:ident [$len:expr] $doc:expr) => {

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[doc = $doc]
        pub struct $name([u8; $len]);

        impl $name {

            // NOTE: `new` or `from_str` function not included in case 
            // there are special constraints on the character types.

            /// NOTE: Assumes NASDAQ only sends valid characters,
            /// and therefore does not check for compliance.
            pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

                use nom::bytes::complete::take;

                let (input, bytes) = take($len)(input)?;
                let chars = bytes.into_iter()
                    .map(|b| *b)
                    .collect::<Vec<u8>>()
                    .try_into()
                    .expect(&format!("Take {} bytes", $len));

                Ok((input, Self(chars)))
            }

            /// Extract the data for inclusion in a message.
            pub fn encode(&self) -> [u8; $len] { self.0 }

            /// Character compliance should be checked when created.
            ///
            /// # Panics
            /// Will panic if the string contains non-UTF8 characters.
            pub fn to_str(&self) -> &str {
                std::str::from_utf8(&self.0)
                    .expect("Character compliance should be checked by type")
                    .trim_end()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(
                &self, 
                f: &mut std::fmt::Formatter<'_>
            ) -> std::fmt::Result {
                self.to_str().fmt(f)
            }
        }

    }
}


define_str!{ FixStr4 [4usize] "Wrappable string type with fixed length 4." }
define_str!{ FixStr8 [8usize] "Wrappable string type with fixed length 8." }
define_str!{ FixStr14 [14usize] "Wrappable string type with fixed length 14." }


use crate::error::TypeError;

define_str!{ 
    Mpid [4usize] 
        "Market Participant Identifier (MPID).
        Used for identifying firms registered with FINRA." 
}

impl Default for Mpid {
    fn default() -> Self { Mpid(*b"    ") }
}

impl Mpid {

    /// Generate a new FirmId from a protocol-compliant string.
    /// Must be four uppercase ASCII characters.
    /// ```
    /// use nsdq_util::Mpid;
    ///
    /// assert!(Mpid::from("FIRM").is_ok());
    /// assert!(Mpid::from("Firm").is_err());
    /// assert!(Mpid::from("F1RM").is_err());
    /// ```
    pub fn from(s: impl AsRef<str>) -> Result<Self, TypeError> {

        let s = s.as_ref();
        if helper::is_uppercase(s) {
            let fs = helper::fixed_str::<4>(s);
            Ok(Mpid(fs))
        } else {
            Err(TypeError::InvalidString(
                String::from("MPID"),
                s.to_string()
            ))
        }
    }
}


define_str!{ 
    StockSymbol [8usize] 
        "Strong type for stock symbols that ensures protocol compliance." 
}

impl Default for StockSymbol {
    fn default() -> Self { StockSymbol(*b"        ") }
}

impl StockSymbol {

    /// Generate a new StockSymbol from a protocol-compliant string.
    /// Must be up to eight alphabetic ASCII characters.
    /// ```
    /// use nsdq_util::StockSymbol;
    ///
    /// assert!(StockSymbol::from("STOCKSYM").is_ok());
    /// assert!(StockSymbol::from("Stonks").is_ok());
    /// assert!(StockSymbol::from("Stonks  ").is_err());
    /// assert!(StockSymbol::from("St0nks").is_err());
    /// ```
    pub fn from(s: impl AsRef<str>) -> Result<Self, TypeError> {

        let s = s.as_ref();
        if helper::is_alpha(s) {
            let fs = helper::fixed_str::<8>(s);
            Ok(StockSymbol(fs))
        } else {
            Err(TypeError::InvalidString(
                String::from("StockSymbol"),
                s.to_string()
            ))
        }
    }
}


pub mod helper {

    /// Creates a fixed-length string, padding up to length `N` with spaces.
    pub fn fixed_str<const N: usize>(s: &str) -> [u8; N] {
        let mut buf = [b' '; N];
        let chars = s.as_bytes();
        let copy_len = usize::min(N, chars.len());
        buf[..copy_len].copy_from_slice(&chars[..copy_len]);
        buf
    }

    /// Checks if all characters are uppercase alpha. (e.g. for Firm ID.)
    pub fn is_uppercase(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_uppercase())
    }

    /// Checks if all characters are alphabetic. (e.g. for stock symbol.)
    pub fn is_alpha(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_alphabetic())
    }

    /// Checks if all characters are alphanumeric or spaces.
    /// (e.g. for CIOrdId)
    /// ```
    /// use nsdq_util::types::string::helper::is_alphanumeric;
    ///
    /// assert!(is_alphanumeric("Xy 1"));
    /// assert!(is_alphanumeric(""));
    /// assert!(!is_alphanumeric("Xy 1!"));
    /// ```
    pub fn is_alphanumeric(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ')
    }
}

