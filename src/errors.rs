use std::fmt::{Display, Formatter};

struct NetError(String);

pub enum Error{
    NetError(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NetError(str) => {
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