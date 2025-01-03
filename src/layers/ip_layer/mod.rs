use ipv4::ipv4_address::IPAddressV4;
use ipv6::ipv6_address::IPAddressV6;

pub mod ip_layer;
pub mod ip_protocol;
pub mod ipv4;
pub mod ipv6;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum IPAddress {
    V4(IPAddressV4),
    V6(IPAddressV6),
}

impl IPAddress {
    pub fn get_bytes(&self) -> Vec<u8> {
        match self {
            IPAddress::V4(ipaddress_v4) => ipaddress_v4.get_bytes().to_vec(),
            IPAddress::V6(ipaddress_v6) => ipaddress_v6.get_bytes().to_vec(),
        }
    }
}

impl Into<IPAddress> for IPAddressV4 {
    fn into(self) -> IPAddress {
        IPAddress::V4(self)
    }
}

impl Into<IPAddress> for IPAddressV6 {
    fn into(self) -> IPAddress {
        IPAddress::V6(self)
    }
}
