use std::fmt::{self, Display, Formatter};

use colored::Colorize;
use eyre::{Context, ContextCompat};

use crate::common::arithmetics::calculate_ones_complement_sum;
use crate::common::formatting::indent_string;
use crate::common::parsing::{read_u16, read_u32, read_u8, read_vec, U13, U3, U4};
use crate::common::proto::Proto;
use crate::layers::ip_layer::ip_protocol::Protocol;
use crate::layers::ip_layer::ipv4::ip_flags::Flags;
use crate::layers::ip_layer::ipv4::type_of_service::TypeOfService;
use crate::layers::transport_layer::transport_layer::TransportLayer;

use super::ipv4_address::IPAddressV4;

#[derive(Clone, Debug)]
pub struct IPv4 {
    pub version: U4,
    pub internet_header_length: U4,
    pub type_of_service: TypeOfService,
    pub total_length: u16,
    pub identification: u16,
    pub flags: Flags,
    pub fragment_offset: U13,
    pub time_to_live: u8,
    pub protocol: Protocol,
    pub header_checksum: u16,
    pub source_address: IPAddressV4,
    pub destination_address: IPAddressV4,
    pub options_and_padding: Vec<u8>,
    pub data: TransportLayer,
}

impl Display for IPv4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IPLayer: {{
    version: {},
    internet_header_length: {},
    type_of_service: {},
    total_length: {},
    identification: {},
    flags: {},
    fragment_offset: {},
    time_to_live: {},
    protocol: {},
    header_checksum: {:#04x},
    source_address: {},
    destination_address: {},
    options_and_Padding: {:?},
    data: {},
}}",
            self.version,
            self.internet_header_length,
            indent_string(self.type_of_service.to_string()),
            self.total_length,
            self.identification,
            indent_string(self.flags.to_string()),
            self.fragment_offset,
            self.time_to_live,
            self.protocol,
            self.header_checksum,
            self.source_address,
            self.destination_address,
            self.options_and_padding,
            indent_string(self.data.to_string()),
        )
    }
}

impl Proto for IPv4 {
    fn to_short_string(&self) -> String {
        format!(
            "{} → {} | {} :: {}",
            self.source_address.to_string().blue(),
            self.destination_address.to_string().purple(),
            self.protocol.to_string().green(),
            self.data.to_short_string()
        )
    }

    fn parse(buf: &mut &[u8]) -> eyre::Result<IPv4> {
        let byte = read_u8(buf).wrap_err("reading first ipv4 byte")?;
        let version: U4 = byte >> 4;
        let internet_header_length: U4 = byte & 0x0F; // Measured in 32 bit segments

        if internet_header_length < 5 {
            eyre::bail!(
                "invalid internet header length {}, minimum is 5",
                internet_header_length
            );
        }
        let mut remaining_header: u16 = match (internet_header_length as u16).checked_mul(32) {
            None => {
                eyre::bail!(
                    "Invalid internet_header_length {} caused overflow on multiply by 32",
                    internet_header_length
                );
            }
            Some(v) => v,
        };

        let total_header_length = remaining_header;
        let data_length;

        let fragment_offset: U13;
        let protocol: Protocol;

        Ok(IPv4 {
            version: {
                remaining_header -= 4;
                version
            },
            internet_header_length: {
                remaining_header -= 4;
                internet_header_length
            },
            type_of_service: {
                remaining_header -= 8;
                let val = read_u8(buf).wrap_err("reading type of service byte")?;
                TypeOfService::parse(val).wrap_err("parsing type of service")?
            },
            total_length: {
                remaining_header -= 16;
                let total_length = read_u16(buf).wrap_err("reading total length")?;

                // Safely calculate the length of the data ((total_length * 8) - total_header_length)
                let total_length_bytes = total_length.checked_mul(8).wrap_err_with(|| {
                    format!("invalid total length {}, overflow occurred", total_length)
                })?;
                data_length = total_length_bytes
                    .checked_sub(total_header_length)
                    .wrap_err_with(|| {
                        format!(
                            "Invalid total length {} with header length {}",
                            total_length, total_header_length
                        )
                    })?;

                total_length
            },
            identification: {
                remaining_header -= 16;
                read_u16(buf).wrap_err("reading identification")?
            },
            flags: {
                let bytes = read_u16(buf).wrap_err("reading flags")?;
                let flags: U3 = (bytes >> 13) as u8;
                fragment_offset = bytes & 0xE0;
                remaining_header -= 3;

                Flags::parse(flags).wrap_err("parsing flags")?
            },
            fragment_offset: {
                remaining_header -= 13;
                fragment_offset
            },
            time_to_live: {
                remaining_header -= 8;
                read_u8(buf).wrap_err("reading TTL")?
            },
            protocol: {
                remaining_header -= 8;
                protocol = Protocol::parse(read_u8(buf).wrap_err("reading protocol")?);
                protocol.clone()
            },
            header_checksum: {
                remaining_header -= 16;
                read_u16(buf).wrap_err("reading header checksum")?
            },
            source_address: {
                remaining_header -= 32;
                let val = read_u32(buf).wrap_err("reading source address")?;
                IPAddressV4(val)
            },
            destination_address: {
                remaining_header -= 32;
                let val = read_u32(buf).wrap_err("reading destination address")?;
                IPAddressV4(val)
            },
            options_and_padding: read_vec(buf, remaining_header as usize)
                .wrap_err("reading options & padding")?,
            data: TransportLayer::parse(&protocol, data_length as usize, buf)
                .wrap_err("parsing transport layer")?,
        })
    }
}

impl IPv4 {
    pub fn generate_response(&self, data: TransportLayer) -> eyre::Result<Self> {
        let internet_header_length: U4 = 5; // TODO: Account for options and padding
        let total_length: u16 = (4 as u16)
            .checked_mul(internet_header_length as u16) // Header length
            .wrap_err("header too large for ipv4")?
            .checked_add(data.len()?) // Add the data length
            .wrap_err("data too large for ipv4")?;

        Ok(IPv4 {
            version: 4,
            internet_header_length: 5, // TODO: Account for options and padding
            type_of_service: TypeOfService::default(),
            total_length,
            identification: 0, // TODO: Implement
            flags: Flags::default(),
            fragment_offset: 0,
            time_to_live: 0b00111100, // As set out in the TCP RFC.
            protocol: Protocol::TCP,
            header_checksum: 0, // TODO: Calculate
            source_address: self.destination_address.clone(),
            destination_address: self.source_address.clone(),
            options_and_padding: vec![], // TODO: lol
            data,
        })
    }

    pub fn serialize(&self) -> eyre::Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let first_byte: u8 = (4 << 4) | 5; // Version 4, header length 5 * 32 bit. TODO: Maybe not hardcoded?
        bytes.push(first_byte);
        bytes.push(self.type_of_service.serialize());
        bytes.extend_from_slice(&self.total_length.to_be_bytes());
        bytes.extend_from_slice(&self.identification.to_be_bytes());
        bytes.extend_from_slice(
            &(((self.flags.serialize() as u16) << 13) | self.fragment_offset).to_be_bytes(),
        );
        bytes.push(self.time_to_live);
        bytes.push(self.protocol.serialize());
        bytes.extend_from_slice(&self.calculate_checksum().to_be_bytes());
        bytes.extend_from_slice(&self.source_address.0.to_be_bytes());
        bytes.extend_from_slice(&self.destination_address.0.to_be_bytes());
        // TODO: Options / Padding
        bytes.extend_from_slice(
            self.data
                .serialize(
                    &self.source_address.clone().into(),
                    &self.destination_address.clone().into(),
                )
                .wrap_err("serializing transport layer data")?
                .as_slice(),
        );
        Ok(bytes)
    }

    pub fn calculate_checksum(&self) -> u16 {
        let mut numbers: Vec<u16> = Vec::new();
        numbers.push(
            ((self.version as u16) << 12)
                | ((self.internet_header_length as u16) << 8)
                | self.type_of_service.serialize() as u16,
        );
        numbers.push(self.total_length);
        numbers.push(self.identification);
        numbers.push(((self.flags.serialize() as u16) << 13) | self.fragment_offset);
        numbers.push(((self.time_to_live as u16) << 8) | (self.protocol.serialize() as u16));
        numbers.push(0 as u16); // Checksum should be 0 for the purpose of the checksum calculation.
        numbers.push((self.source_address.0 >> 16) as u16);
        numbers.push(self.source_address.0 as u16);
        numbers.push((self.destination_address.0 >> 16) as u16);
        numbers.push(self.destination_address.0 as u16);
        // TODO: Support options
        // TODO: Add data

        return calculate_ones_complement_sum(numbers);
    }
}
