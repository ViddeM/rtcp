use std::fmt::Display;

use colored::Colorize;
use eyre::{Context, ContextCompat};

use crate::{
    common::{
        formatting::indent_string,
        parsing::{read_u128, read_u16, read_u8, U20, U4},
        proto::Proto,
    },
    layers::{ip_layer::ip_protocol::Protocol, transport_layer::transport_layer::TransportLayer},
};

use super::ipv6_address::IPAddressV6;

#[derive(Clone, Debug)]
pub struct IPv6 {
    pub version: U4,
    pub traffic_class: u8,
    pub flow_label: U20,
    pub payload_length: u16,
    pub next_header: Protocol,
    pub hop_limit: u8,
    pub source_address: IPAddressV6,
    pub destination_address: IPAddressV6,
    pub data: TransportLayer,
}

impl Display for IPv6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IPLayer: {{
    version: {},
    traffic_class: {},
    flow_label: {},
    payload_length: {},
    next_header: {},
    hop_limit: {},
    source_address: {},
    destination_address: {},
    data: {}
}}",
            self.version,
            self.traffic_class,
            self.flow_label,
            self.payload_length,
            indent_string(self.next_header.to_string()),
            self.hop_limit,
            self.source_address,
            self.destination_address,
            indent_string(self.data.to_string())
        )
    }
}

impl Proto for IPv6 {
    fn to_short_string(&self) -> String {
        format!(
            "{} → {} | {} :: {}",
            self.source_address.to_string().blue(),
            self.destination_address.to_string().purple(),
            self.next_header.to_string().green(),
            self.data.to_short_string()
        )
    }

    fn parse(buf: &mut &[u8]) -> eyre::Result<Self> {
        let byte = read_u8(buf).wrap_err("reading version byte")?;
        let version: U4 = byte >> 4;

        let next_byte = read_u8(buf).wrap_err("reading second IPv6 byte")?;

        let traffic_class = (byte << 4) | (next_byte >> 4);

        let flow_label = read_u16(buf).wrap_err("reading flow_label")? as U20;
        let flow_label = (((next_byte & 0x0F) as U20) << 16) | flow_label;

        let payload_length = read_u16(buf).wrap_err("reading payload length")?;

        let protocol = Protocol::parse(read_u8(buf).wrap_err("reading protocol byte")?);

        Ok(IPv6 {
            version,
            traffic_class,
            flow_label,
            payload_length,
            next_header: protocol.clone(),
            hop_limit: read_u8(buf).wrap_err("reading hop limit")?,
            source_address: IPAddressV6(read_u128(buf).wrap_err("reading source address")?),
            destination_address: IPAddressV6(
                read_u128(buf).wrap_err("reading destination address")?,
            ),
            data: TransportLayer::parse(&protocol, payload_length as usize, buf)
                .wrap_err("parsing transport layer")?,
        })
    }
}

impl IPv6 {
    pub fn generate_response(&self, data: TransportLayer) -> eyre::Result<Self> {
        todo!("not yet implemented")
    }
}
