use crate::common::parsing::{read_u16, read_u32, read_u8, read_vec, U13, U3, U4};
use crate::layers::ip_layer::ip_address::IPAddress;
use crate::layers::ip_layer::type_of_service::TypeOfService;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::common::formatting::indent_string;
use crate::layers::ip_layer::ip_flags::Flags;
use crate::layers::ip_layer::ip_protocol::Protocol;

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
}

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
    pub source_address: IPAddress,
    pub destination_address: IPAddress,
    pub options_and_padding: Vec<u8>,
    pub data: Vec<u8>,
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
        )
    }
}

impl IPv4 {
    pub fn to_short_string(&self) -> String {
        format!("{} → {} | {}b of {}", self.source_address, self.destination_address, self.data.len(), self.protocol)
    }

    pub fn parse(buf: &mut &[u8]) -> Option<IPv4> {
        let byte = read_u8(buf)?;
        let version: U4 = byte >> 4;
        let internet_header_length: U4 = byte & 0x0F; // Measured in 32 bit segments

        if internet_header_length < 5 {
            eprintln!(
                "Invalid internet header length {}, minimum is 5",
                internet_header_length
            );
            return None;
        }
        let mut remaining_header: u16 = match (internet_header_length as u16).checked_mul(32) {
            None => {
                eprintln!(
                    "Invalid internet_header_length {} caused overflow on multiply by 32",
                    internet_header_length
                );
                return None;
            }
            Some(v) => v,
        };
        let total_header_length = remaining_header;
        let data_length;

        let fragment_offset: U13;

        Some(IPv4 {
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
                let val = read_u8(buf)?;
                TypeOfService::parse(val)?
            },
            total_length: {
                remaining_header -= 16;
                let total_length = read_u16(buf)?;

                // Safely calculate the length of the data ((total_length * 8) - total_header_length)
                match total_length.checked_mul(8) {
                    None => {
                        eprintln!("Invalid total length {}, overflow occured", total_length);
                        return None;
                    }
                    Some(v) => match v.checked_sub(total_header_length) {
                        None => {
                            eprintln!(
                                "Invalid total length, {} with header length {}",
                                v, total_header_length
                            );
                            return None;
                        }
                        Some(data_len) => data_length = data_len,
                    },
                }

                total_length
            },
            identification: {
                remaining_header -= 16;
                read_u16(buf)?
            },
            flags: {
                let bytes = read_u16(buf)?;
                let flags: U3 = (bytes >> 13) as u8;
                fragment_offset = bytes & 0xE0;
                remaining_header -= 3;

                Flags::parse(flags)?
            },
            fragment_offset: {
                remaining_header -= 13;
                fragment_offset
            },
            time_to_live: {
                remaining_header -= 8;
                read_u8(buf)?
            },
            protocol: {
                remaining_header -= 8;
                Protocol::parse(read_u8(buf)?)
            },
            header_checksum: {
                remaining_header -= 16;
                read_u16(buf)?
            },
            source_address: {
                remaining_header -= 32;
                let val = read_u32(buf)?;
                IPAddress(val)
            },
            destination_address: {
                remaining_header -= 32;
                let val = read_u32(buf)?;
                IPAddress(val)
            },
            options_and_padding: read_vec(buf, remaining_header as usize)?,
            data: read_vec(buf, data_length as usize)?,
        })
    }
}
