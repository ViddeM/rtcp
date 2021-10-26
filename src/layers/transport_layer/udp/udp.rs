use crate::common::arithmetics::calculate_ones_complement_sum;
use crate::common::parsing::read_u16;
use crate::common::response_error::ResponseError;
use crate::layers::ip_layer::ipv4::ip_address::IPAddress;
use crate::layers::ip_layer::ipv4::ip_protocol::Protocol;
use core::fmt;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct UDP {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
    pub data: Vec<u8>,
}

impl UDP {
    pub fn to_short_string(&self) -> String {
        format!(
            ":{} -> :{} {}b ({}b) checksum:{}",
            self.src_port,
            self.dst_port,
            self.length,
            self.data.len(),
            self.checksum
        )
    }

    pub fn parse(buf: &mut &[u8]) -> Option<UDP> {
        Some(UDP {
            src_port: read_u16(buf)?,
            dst_port: read_u16(buf)?,
            length: read_u16(buf)?,
            checksum: read_u16(buf)?,
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
        bytes.extend_from_slice(&self.len()?.to_be_bytes());
        bytes.extend_from_slice(&self.calculate_checksum(src_adr, dst_adr)?.to_be_bytes());

        bytes.extend_from_slice(&self.data.as_slice());

        Ok(bytes)
    }

    pub fn len(&self) -> Result<u16, ResponseError> {
        const header_len: u16 = 8; // Always 8 bytes
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
        // 96 bit pseudo header
        num.push((src_adr.0 >> 16) as u16);
        num.push(src_adr.0 as u16);
        num.push((dst_adr.0 >> 16) as u16);
        num.push(dst_adr.0 as u16);
        num.push(Protocol::UDP.serialize() as u16);
        num.push(self.length);

        // UDP header
        num.push(self.src_port);
        num.push(self.dst_port);
        num.push(self.length);
        num.push(0);

        // Data
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

impl Display for UDP {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{
    Source port: {},
    Destination port: {},
    Length: {},
    Checksum: {:X},
    Data: {:?}
        
        }}",
            self.src_port, self.dst_port, self.length, self.checksum, self.data
        )
    }
}
