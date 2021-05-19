use crate::layers::ip_layer::ipv4::ipv4::IPv4;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::common::response_error::ResponseError;

#[derive(Clone, Debug)]
pub enum IPLayerProtocol {
    IPv4(IPv4),
    Other(Vec<u8>),
}

impl Display for IPLayerProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IPLayerProtocol::IPv4(ipv4) => write!(f, "ipv4: {}", ipv4),
            IPLayerProtocol::Other(bytes) => write!(f, "other: {:?}", bytes),
        }
    }
}

impl IPLayerProtocol {
    pub fn parse(bytes: &mut &[u8]) -> IPLayerProtocol {
        match IPv4::parse(bytes) {
            Some(ipv4) => IPLayerProtocol::IPv4(ipv4),
            None => IPLayerProtocol::Other(bytes.to_vec()),
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, ResponseError> {
        match self {
            IPLayerProtocol::IPv4(ipv4) => ipv4.serialize(),
            IPLayerProtocol::Other(data) => Ok(data.to_vec()),
        }
    }
}