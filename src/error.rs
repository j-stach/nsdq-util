
/// 
#[derive(Debug)]
pub enum TypeError {
    InvalidPrice(String),
    InvalidString(String, String),
    InvalidTime(u64),
}

use std::fmt;

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let msg = match self {
            TypeError::InvalidPrice(val) => format!("Price: {}", val),
            TypeError::InvalidString(k, val) => format!("{}: {}", k, val),
            TypeError::InvalidTime(val) => format!("Time: {} (nanosec)", val),
        };
        
        write!(f, "Invalid Type - {}", msg)
    }
}

impl std::error::Error for TypeError {}

