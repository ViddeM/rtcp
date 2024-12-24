use crate::common::parsing::read_vec;
use crate::common::response_error::ResponseError;
use crate::layers::ip_layer::ipv4::ip_address::IPAddress;
use crate::layers::ip_layer::ipv4::ip_protocol::Protocol;
use crate::layers::transport_layer::tcp::tcp::TCP;
use crate::layers::transport_layer::udp::udp::UDP;
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Mul;

#[derive(Clone, Debug)]
pub enum TransportLayer {
    TCP(TCP),
    UDP(UDP),
    Other(Vec<u8>),
}

impl Display for TransportLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TransportLayer::TCP(tcp) => write!(f, "TCP: {}", tcp),
            TransportLayer::UDP(udp) => write!(f, "UDP: {}", udp),
            TransportLayer::Other(v) => write!(f, "{} bytes (unsupported)", v.len()),
        }
    }
}

impl TransportLayer {
    pub fn to_short_string(&self) -> String {
        match self {
            TransportLayer::TCP(tcp) => tcp.to_short_string(),
            TransportLayer::UDP(udp) => udp.to_short_string(),
            TransportLayer::Other(d) => format!("{}b", d.len()),
        }
    }

    pub fn serialize(
        &self,
        src_adr: &IPAddress,
        dst_adr: &IPAddress,
    ) -> Result<Vec<u8>, ResponseError> {
        Ok(match self {
            TransportLayer::TCP(tcp) => tcp.serialize(src_adr, dst_adr)?,
            TransportLayer::UDP(udp) => udp.serialize(src_adr, dst_adr)?,
            TransportLayer::Other(data) => data.to_vec(),
        })
    }

    pub fn len(&self) -> Result<u16, ResponseError> {
        Ok(match &self {
            TransportLayer::TCP(tcp) => tcp.len()?,
            TransportLayer::UDP(udp) => udp.len()?,
            TransportLayer::Other(data) => data.len() as u16,
        })
    }

    pub fn parse(protocol: &Protocol, len: usize, buf: &mut &[u8]) -> Option<Self> {
        Some(match protocol {
            Protocol::TCP => Self::TCP(TCP::parse(buf)?),
            Protocol::UDP => Self::UDP(UDP::parse(buf)?),
            _ => Self::Other(read_vec(buf, len)?),
        })
    }
}
