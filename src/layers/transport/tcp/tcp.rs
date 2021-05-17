use crate::common::parsing::{U4, U6, read_u16, read_u32, read_vec};
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::common::formatting::indent_string;
use crate::layers::transport::tcp::control_bits::ControlBits;

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
    pub data: Vec<u8>
}

// The minimum length (measured in 32-bit segments) of the TCP header,
// i.e. the entire header excluding any options & padding.
const TCP_MIN_HEADER_LENGTH: U4 = 5;

impl TCP {
    pub fn to_short_string(&self) -> String {
        format!(":{} → :{} [{}] {}b", self.src_port, self.dst_port, self.control_bits.to_short_string(), self.data.len())
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
                    eprintln!("Invalid data_offset number: {} < {}", data_offset, TCP_MIN_HEADER_LENGTH);
                    return None
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
}

impl Display for TCP {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{
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
}}", self.src_port, self.dst_port, self.sequence_number, self.acknowledgement_number,
            self.data_offset, self.reserved, indent_string(self.control_bits.to_string()),
            self.window, self.urgent_pointer, self.options, self.data,
        )
    }
}