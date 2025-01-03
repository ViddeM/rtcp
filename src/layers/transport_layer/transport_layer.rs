use eyre::{Context, ContextCompat};

use crate::common::parsing::read_vec;
use crate::common::proto::Proto;
use crate::layers::ip_layer::ip_protocol::Protocol;
use crate::layers::ip_layer::IPAddress;
use crate::layers::transport_layer::tcp::tcp::TCP;
use crate::layers::transport_layer::udp::udp::UDP;
use std::fmt;
use std::fmt::{Display, Formatter};

use super::icmpv6::icmpv6::ICMPv6;

#[derive(Clone, Debug)]
pub enum TransportLayer {
    TCP(TCP),
    UDP(UDP),
    ICMPv6(ICMPv6),
    Other(Vec<u8>),
}

impl Display for TransportLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TransportLayer::TCP(tcp) => write!(f, "TCP: {}", tcp),
            TransportLayer::UDP(udp) => write!(f, "UDP: {}", udp),
            TransportLayer::ICMPv6(icmpv6) => write!(f, "ICMP (v6): {}", icmpv6),
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
            TransportLayer::ICMPv6(icmpv6) => icmpv6.to_short_string(),
        }
    }

    pub fn serialize(&self, src_adr: &IPAddress, dst_adr: &IPAddress) -> eyre::Result<Vec<u8>> {
        Ok(match self {
            TransportLayer::TCP(tcp) => tcp
                .serialize(src_adr, dst_adr)
                .wrap_err("failed to serialize TCP")?,
            TransportLayer::UDP(udp) => udp
                .serialize(src_adr, dst_adr)
                .wrap_err("failed to serialize UDP")?,
            TransportLayer::ICMPv6(_icmpv6) => todo!("Not implemented"),
            TransportLayer::Other(data) => data.to_vec(),
        })
    }

    pub fn len(&self) -> eyre::Result<u16> {
        Ok(match &self {
            TransportLayer::TCP(tcp) => tcp.len().wrap_err("failed getting TCP length")?,
            TransportLayer::UDP(udp) => udp.len().wrap_err("failed getting UDP length")?,
            TransportLayer::ICMPv6(_icmpv6) => todo!("Not implemented"),
            TransportLayer::Other(data) => data.len() as u16,
        })
    }

    pub fn parse(protocol: &Protocol, len: usize, buf: &mut &[u8]) -> eyre::Result<Self> {
        Ok(match protocol {
            Protocol::TCP => Self::TCP(TCP::parse(buf).wrap_err("TCP parsing failed")?),
            Protocol::UDP => Self::UDP(UDP::parse(buf).wrap_err("UDP parsing failed")?),
            Protocol::IPv6ICMP => {
                Self::ICMPv6(ICMPv6::parse(buf).wrap_err("failed parsing ICMP v6")?)
            }
            _ => Self::Other(read_vec(buf, len).wrap_err("failed reading unknown bytes")?),
        })
    }
}
