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

impl TransportLayer {
    pub fn to_short_string(&self) -> String {
        match self {
            TransportLayer::TCP(tcp) => tcp.to_short_string(),
            TransportLayer::Other(d) => format!("{}b {:?}", d.len(), d),
        }
    }
}