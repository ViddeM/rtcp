use eyre::{Context, ContextCompat};

use crate::common::formatting::indent_string;
use crate::common::parsing::read_u16;
use crate::common::proto::Proto;
use crate::layers::ip_layer::ip_layer::IPLayerProtocol;
use std::fmt;
use std::fmt::{Display, Formatter};

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
            self.flags,
            self.proto,
            indent_string(self.data.to_string()),
        )
    }
}

impl Proto for TunLayer {
    fn to_short_string(&self) -> String {
        format!("{} ({})", self.proto, self.flags)
    }

    fn parse(buf: &mut &[u8]) -> eyre::Result<Self> {
        let t = TunLayer {
            flags: read_u16(buf).wrap_err("reading flags")?,
            proto: Protocol::parse(read_u16(buf).wrap_err("reading protocol")?),
            data: IPLayerProtocol::parse(buf),
        };

        match (&t.proto, &t.data) {
            (Protocol::IPv4, IPLayerProtocol::IPv4(_)) => {}
            (Protocol::IPv6, IPLayerProtocol::IPv6(_)) => {}
            (proto, parsed) => {
                eyre::bail!("Missmatched protocol, expected {proto}, got {parsed}");
            }
        }

        Ok(t)
    }
}

impl TunLayer {
    pub fn generate_response(ip_layer: IPLayerProtocol) -> TunLayer {
        TunLayer {
            flags: 0,
            proto: Protocol::IPv4,
            data: ip_layer,
        }
    }

    pub fn serialize(&self) -> eyre::Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.flags.to_be_bytes());
        bytes.extend_from_slice(&self.proto.serialize().to_be_bytes());
        bytes.extend_from_slice(&self.data.serialize().wrap_err("serializing data")?);
        Ok(bytes)
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

const PROTO_IPV4: u16 = 0x0800;
const PROTO_ARP: u16 = 0x0806;
const PROTO_WAKE_ON_LAN: u16 = 0x0842;
const PROTO_APPLE_TALK: u16 = 0x809B;
const PROTO_AARP: u16 = 0x80F3;
const PROTO_SLPP: u16 = 0x8102;
const PROTO_IPV6: u16 = 0x86DD;
const PROTO_ETHERNET_FLOW_CONTROL: u16 = 0x8808;

impl Protocol {
    fn parse(num: u16) -> Protocol {
        match num {
            PROTO_IPV4 => Protocol::IPv4,
            PROTO_ARP => Protocol::ARP,
            PROTO_WAKE_ON_LAN => Protocol::WakeOnLAN,
            PROTO_APPLE_TALK => Protocol::AppleTalk,
            PROTO_AARP => Protocol::AARP,
            PROTO_SLPP => Protocol::SLPP,
            PROTO_IPV6 => Protocol::IPv6,
            PROTO_ETHERNET_FLOW_CONTROL => Protocol::EthernetFlowControl,
            val => Protocol::Unknown(val),
        }
    }

    fn serialize(&self) -> u16 {
        match self {
            Protocol::IPv4 => PROTO_IPV4,
            Protocol::ARP => PROTO_ARP,
            Protocol::WakeOnLAN => PROTO_WAKE_ON_LAN,
            Protocol::AppleTalk => PROTO_APPLE_TALK,
            Protocol::AARP => PROTO_AARP,
            Protocol::SLPP => PROTO_SLPP,
            Protocol::IPv6 => PROTO_IPV6,
            Protocol::EthernetFlowControl => PROTO_ETHERNET_FLOW_CONTROL,
            Protocol::Unknown(v) => v.clone(),
        }
    }
}
