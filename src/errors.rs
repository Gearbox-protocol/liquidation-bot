use std::fmt::{Display, Formatter};
use std::error;

struct NetError(String);

#[derive(Debug)]
pub enum LiquidationError {
    NetError(String),
}

impl Display for LiquidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiquidationError::NetError(str) => {
                write!(f, "{}", str)
            }

            _ => {
                write!(f, "unknown error")
            }
        }
    }
}

impl Display for NetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
