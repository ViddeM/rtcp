use crate::common::parsing::{read_u16};
use crate::layers::ip_layer::ip_layer::IPLayerProtocol;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::common::formatting::indent_string;

#[derive(Clone, Debug)]
pub struct TunLayer {
    pub flags: u16,
    pub proto: Protocol,
    pub data: IPLayerProtocol,
}

impl Display for TunLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tunlayer: {{
    flags: {:x},
    proto: {},
    data: {},
}}",
            self.flags, self.proto, indent_string(self.data.to_string()),
        )
    }
}

#[derive(Clone, Debug)]
pub enum Protocol {
    IPv4,
    ARP,
    WakeOnLAN,
    AppleTalk,
    AARP,
    SLPP,
    IPv6,
    EthernetFlowControl,
    Unknown(u16),
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::IPv4 => write!(f, "Internet Protocol version 4 (IPv4)"),
            Protocol::ARP => write!(f, "Address Resolution Protocol (ARP)"),
            Protocol::WakeOnLAN => write!(f, "Wake-on-LAN"),
            Protocol::AppleTalk => write!(f, "AppleTalk"),
            Protocol::AARP => write!(f, "AppleTalk Address Resolution Protocol (AARP)"),
            Protocol::SLPP => write!(f, "Simple Loop Prevention Protocol (SLPP)"),
            Protocol::IPv6 => write!(f, "Internet Protocol version 6 (IPv6)"),
            Protocol::EthernetFlowControl => write!(f, "Ethernet Flow Control"),
            Protocol::Unknown(v) => write!(f, "Unknown ({:x})", v),
        }
    }
}

impl Protocol {
    fn parse(num: u16) -> Protocol {
        match num {
            0x0800 => Protocol::IPv4,
            0x0806 => Protocol::ARP,
            0x0842 => Protocol::WakeOnLAN,
            0x809B => Protocol::AppleTalk,
            0x80F3 => Protocol::AARP,
            0x8102 => Protocol::SLPP,
            0x86DD => Protocol::IPv6,
            0x8808 => Protocol::EthernetFlowControl,
            val => Protocol::Unknown(val),
        }
    }
}

pub fn parse_tun_layer(buf: &mut &[u8]) -> Option<TunLayer> {
    let proto: Protocol;
    Some(TunLayer {
        flags: read_u16(buf)?,
        proto: {
            proto = Protocol::parse(read_u16(buf)?);
            proto.clone()
        },
        data: {
            match proto {
                Protocol::IPv4 => IPLayerProtocol::parse(buf),
                _ => IPLayerProtocol::Other(buf.to_vec()),
            }
        },
    })
}
