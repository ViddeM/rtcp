use crate::common::arithmetics::calculate_ones_complement_sum;
use crate::common::formatting::indent_string;
use crate::common::parsing::{read_u16, read_u32, read_vec, U4, U6};
use crate::common::response_error::ResponseError;
use crate::layers::ip_layer::ipv4::ip_address::IPAddress;
use crate::layers::ip_layer::ipv4::ip_protocol::Protocol;
use crate::layers::transport_layer::tcp::control_bits::ControlBits;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct TCP {
    pub src_port: u16,
    pub dst_port: u16,
    pub sequence_number: u32,
    pub acknowledgement_number: u32,
    pub data_offset: U4,
    pub reserved: U6,
    pub control_bits: ControlBits,
    pub window: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    pub options: Vec<u8>,
    pub data: Vec<u8>,
}

// The minimum length (measured in 32-bit segments) of the TCP header,
// i.e. the entire header excluding any options & padding.
const TCP_MIN_HEADER_LENGTH: U4 = 5;

impl TCP {
    pub fn to_short_string(&self) -> String {
        format!(
            ":{} → :{} [{}] {}b",
            self.src_port,
            self.dst_port,
            self.control_bits.to_short_string(),
            self.data.len()
        )
    }

    pub fn parse(buf: &mut &[u8]) -> Option<TCP> {
        let offset_reserved_control_bits;
        let data_offset: U4;
        Some(TCP {
            src_port: read_u16(buf)?,
            dst_port: read_u16(buf)?,
            sequence_number: read_u32(buf)?,
            acknowledgement_number: read_u32(buf)?,
            data_offset: {
                offset_reserved_control_bits = read_u16(buf)?;
                data_offset = ((offset_reserved_control_bits & 0xF000) >> 12) as U4;
                if data_offset < TCP_MIN_HEADER_LENGTH {
                    eprintln!(
                        "Invalid data_offset number: {} < {}",
                        data_offset, TCP_MIN_HEADER_LENGTH
                    );
                    return None;
                }
                data_offset
            },
            reserved: ((offset_reserved_control_bits & 0x0FC0) >> 6) as U6,
            control_bits: ControlBits::parse((offset_reserved_control_bits & 0x003F) as U6),
            window: read_u16(buf)?,
            checksum: read_u16(buf)?,
            urgent_pointer: read_u16(buf)?,
            options: read_vec(buf, (data_offset - TCP_MIN_HEADER_LENGTH) as usize)?,
            data: buf.to_vec(),
        })
    }

    pub fn serialize(
        &self,
        src_adr: &IPAddress,
        dst_adr: &IPAddress,
    ) -> Result<Vec<u8>, ResponseError> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.src_port.to_be_bytes());
        bytes.extend_from_slice(&self.dst_port.to_be_bytes());
        bytes.extend_from_slice(&self.sequence_number.to_be_bytes());
        bytes.extend_from_slice(&self.acknowledgement_number.to_be_bytes());
        bytes.extend_from_slice(
            &(0 | ((self.data_offset as u16) << 12) | (self.control_bits.serialize() as u16))
                .to_be_bytes(),
        );
        bytes.extend_from_slice(&self.window.to_be_bytes());
        bytes.extend_from_slice(&self.calculate_checksum(src_adr, dst_adr)?.to_be_bytes());
        bytes.extend_from_slice(&self.urgent_pointer.to_be_bytes());
        // TODO: Add options / padding
        bytes.extend_from_slice(&self.options);

        bytes.extend_from_slice(self.data.as_slice());

        Ok(bytes)
    }

    // Calculates the full length of this TCP packet, i.e. Header + Data
    pub fn len(&self) -> Result<u16, ResponseError> {
        let header_len = (self.data_offset as u16)
            .checked_mul(4) // Convert no 32bit words to no bytes.
            .ok_or(ResponseError::DataTooLarge)?
            + (self.options.len() as u16);

        let data_len = self.data.len() as u16;

        header_len
            .checked_add(data_len)
            .ok_or(ResponseError::DataTooLarge)
    }

    pub fn calculate_checksum(
        &self,
        src_adr: &IPAddress,
        dst_adr: &IPAddress,
    ) -> Result<u16, ResponseError> {
        let mut num: Vec<u16> = Vec::new();

        // 96 bit Pseudo header
        num.push((src_adr.0 >> 16) as u16);
        num.push(src_adr.0 as u16);
        num.push((dst_adr.0 >> 16) as u16);
        num.push(dst_adr.0 as u16);
        num.push(0 as u16); // Zeros -- Is this really correct?
        num.push(Protocol::TCP.serialize() as u16); // Protocol
        num.push(self.len()?); // Full length

        // Actual data for checksum
        num.push(self.src_port);
        num.push(self.dst_port);
        num.push((self.sequence_number >> 16) as u16);
        num.push(self.sequence_number as u16);
        num.push((self.acknowledgement_number >> 16) as u16);
        num.push(self.acknowledgement_number as u16);
        num.push(
            ((self.data_offset as u16) << 12)
                | (self.reserved << 6) as u16
                | self.control_bits.serialize() as u16,
        );
        num.push(self.window);
        num.push(0 as u16);
        num.push(self.urgent_pointer);
        // TODO: Support options/padding

        for (index, val) in self.data.iter().enumerate() {
            if index % 2 == 0 {
                let next_num: u8 = if index < self.data.len() - 1 {
                    self.data[index + 1]
                } else {
                    0 // If our data is not an even number of 16-bit words we need to pad it with 0s.
                };
                num.push(((val.clone() as u16) << 8) | next_num as u16);
            }
        }

        Ok(calculate_ones_complement_sum(num))
    }
}

impl Display for TCP {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{
    Source port: {},
    Destination port: {},
    Sequence number: {},
    Acknowledgement number: {},
    Data offset: {},
    Reserved: {},
    Control Bits: {},
    Window: {},
    Urgent pointer: {},
    Options: {:?},
    Data: {:?},
}}",
            self.src_port,
            self.dst_port,
            self.sequence_number,
            self.acknowledgement_number,
            self.data_offset,
            self.reserved,
            indent_string(self.control_bits.to_string()),
            self.window,
            self.urgent_pointer,
            self.options,
            self.data,
        )
    }
}
