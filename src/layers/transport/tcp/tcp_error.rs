use std::fmt::{Display, Formatter};
use std::fmt;

pub enum TcpError {
    UnexpectedConnection,
}

impl Display for TcpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TcpError::UnexpectedConnection => write!(f, "Unexpected connection"),
        }
    }
}