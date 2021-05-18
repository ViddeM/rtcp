use crate::common::parsing::U6;
use std::fmt::{Display, Formatter};
use std::fmt;

const DELIMITER: &str = ", ";
const URG_BIT: u8 = 0b00100000;
const ACK_BIT: u8 = 0b00010000;
const PSH_BIT: u8 = 0b00001000;
const RST_BIT: u8 = 0b00000100;
const SYN_BIT: u8 = 0b00000010;
const FIN_BIT: u8 = 0b00000001;

#[derive(Clone, Debug)]
pub struct ControlBits {
    pub urg: bool,
    pub ack: bool,
    pub psh: bool,
    pub rst: bool,
    pub syn: bool,
    pub fin: bool,
}

impl ControlBits {
    fn get_flags_as_vec(&self) -> Vec<&str> {
        let mut flags = Vec::new();

        if self.urg {flags.push("URG")}
        if self.ack {flags.push("ACK")}
        if self.psh {flags.push("PSH")}
        if self.rst {flags.push("RST")}
        if self.syn {flags.push("SYN")}
        if self.fin {flags.push("FIN")}

        return flags
    }

    pub fn to_short_string(&self) -> String {
        let flags = self.get_flags_as_vec();
        let mut str = String::new();

        for (index, flag) in flags.iter().enumerate() {
            str += flag;
            if index < flags.len() - 1 {
                str += DELIMITER;
            }
        }

        return str
    }

    pub fn parse(num: U6) -> ControlBits {
        ControlBits {
            urg: (num & URG_BIT) >> 5 == 1,
            ack: (num & ACK_BIT) >> 4 == 1,
            psh: (num & PSH_BIT) >> 3 == 1,
            rst: (num & RST_BIT) >> 2 == 1,
            syn: (num & SYN_BIT) >> 1 == 1,
            fin: num & FIN_BIT == 1,
        }
    }

    pub fn serialize(&self) -> U6 {
        let mut num = 0;
        if self.urg { num = num | URG_BIT }
        if self.ack { num = num | ACK_BIT }
        if self.psh { num = num | PSH_BIT }
        if self.rst { num = num | RST_BIT }
        if self.syn { num = num | SYN_BIT }
        if self.fin { num = num | FIN_BIT }
        return num
    }

    pub fn get_syn() -> ControlBits {
        ControlBits {
            urg: false,
            ack: false,
            psh: false,
            rst: false,
            syn: true,
            fin: false
        }
    }

    pub fn get_syn_ack() -> ControlBits {
        ControlBits {
            urg: false,
            ack: true,
            psh: false,
            rst: false,
            syn: true,
            fin: false
        }
    }
}

impl Display for ControlBits {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let flags = self.get_flags_as_vec();

        let mut bits = String::new();
        for (index, flag) in flags.iter().enumerate() {
            bits += flag;
            if index < flags.len() - 1 {
                bits += DELIMITER
            }
        }

        write!(f, "{{ {} }}", bits)
    }
}