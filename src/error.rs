
/// 
#[derive(Debug)]
pub enum TypeError {
    InvalidPrice(String),
    InvalidString(String, String),
}

use std::fmt;

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let msg = match self {
            TypeError::InvalidPrice(val) => format!("Price: {}", val),
            TypeError::InvalidString(k, val) => format!("{}: {}", k, val),
        };
        
        write!(f, "Invalid Type - {}", msg)
    }
}

impl std::error::Error for TypeError {}

