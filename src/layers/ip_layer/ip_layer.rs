use crate::common::response_error::ResponseError;
use crate::layers::ip_layer::ipv4::ipv4::IPv4;
use std::fmt;
use std::fmt::{Display, Formatter};

use super::ipv6::ipv6::IPv6;

#[derive(Clone, Debug)]
pub enum IPLayerProtocol {
    IPv4(IPv4),
    IPv6(IPv6),
    Other(Vec<u8>),
}

impl Display for IPLayerProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IPLayerProtocol::IPv4(ipv4) => write!(f, "ipv4: {}", ipv4),
            IPLayerProtocol::Other(bytes) => write!(f, "other: {:?}", bytes),
            IPLayerProtocol::IPv6(ipv6) => write!(f, "ipv6: {}", ipv6),
        }
    }
}

impl IPLayerProtocol {
    pub fn parse(bytes: &mut &[u8]) -> IPLayerProtocol {
        let first_byte = bytes[0];
        let version = (first_byte & 0xf0) >> 4;

        match version {
            4 => IPLayerProtocol::IPv4(IPv4::parse(bytes).expect("parsing ipv4")),
            6 => IPLayerProtocol::IPv6(IPv6::parse(bytes).expect("parsing ipv6")),
            _ => IPLayerProtocol::Other(bytes.to_vec()),
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, ResponseError> {
        match self {
            IPLayerProtocol::IPv4(ipv4) => ipv4.serialize(),
            IPLayerProtocol::Other(data) => Ok(data.to_vec()),
            IPLayerProtocol::IPv6(ipv6) => todo!(),
        }
    }
}
