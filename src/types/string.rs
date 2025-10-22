
// TODO Docs example
///
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
            pub fn to_string(&self) -> &str {
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
                self.to_string().fmt(f)
            }
        }

    }
}


define_str!{ FixStr4 [4usize] "Wrappable string type with fixed length 4." }
define_str!{ FixStr8 [8usize] "Wrappable string type with fixed length 8." }
define_str!{ FixStr14 [14usize] "Wrappable string type with fixed length 14." }


define_str!{ 
    FirmId [4usize] 
        "Strong type for firm IDs that ensures protocol compliance." 
}

impl Default for FirmId {
    fn default() -> Self { FirmId(*b"    ") }
}

impl FirmId {

    // TODO Error type
    /// Generate a new FirmId from a protocol-compliant string.
    pub fn from(s: impl AsRef<str>) -> Option<Self> {

        let s = s.as_ref();
        if helper::is_uppercase(s) {
            let fs = helper::fixed_str::<4>(s);
            Some(FirmId(fs))
        } else {
            None
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

    // TODO Error type
    /// Generate a new StockSymbol from a protocol-compliant string.
    pub fn from(s: impl AsRef<str>) -> Option<Self> {

        let s = s.as_ref();
        if helper::is_alpha(s) {
            let fs = helper::fixed_str::<8>(s);
            Some(StockSymbol(fs))
        } else {
            None
        }
    }
}


pub mod helper {

    /// Pads a string up to length `N` with spaces.
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

    /// Checks if all characters are alphanumeric or whitespace.
    /// (e.g. for CIOrdId)
    pub fn is_alphanumeric(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ')
    }
}

