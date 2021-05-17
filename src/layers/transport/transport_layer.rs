use crate::layers::transport::tcp::tcp::TCP;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Clone, Debug)]
pub enum TransportLayer {
    TCP(TCP),
    Other(Vec<u8>),
}

impl Display for TransportLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TransportLayer::TCP(tcp) => write!(f, "TCP: {}", tcp),
            TransportLayer::Other(v) => write!(f, "{} bytes (unsupported)", v.len()),
        }
    }
}