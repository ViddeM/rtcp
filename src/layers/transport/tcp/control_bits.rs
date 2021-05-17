use crate::common::parsing::U6;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Clone, Debug)]
pub struct ControlBits {
    urg: bool,
    ack: bool,
    psh: bool,
    rst: bool,
    syn: bool,
    fin: bool,
}

impl ControlBits {
    pub fn parse(num: U6) -> ControlBits {
        ControlBits {
            urg: num & 0b00100000 == 1,
            ack: num & 0b00010000 == 1,
            psh: num & 0b00001000 == 1,
            rst: num & 0b00000100 == 1,
            syn: num & 0b00000010 == 1,
            fin: num & 0b00000001 == 1
        }
    }
}

const DELIMITER: &str = " | ";
impl Display for ControlBits {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();

        if self.urg {flags.push("URG")}
        if self.ack {flags.push("ACK")}
        if self.psh {flags.push("PSH")}
        if self.rst {flags.push("RST")}
        if self.syn {flags.push("SYN")}
        if self.fin {flags.push("FIN")}

        let mut bits = String::new();
        for (index, flag) in flags.iter().enumerate() {
            bits += flag;
            if index < flags.len() - 1 {
                bits += DELIMITER
            }
        }

        write!(f, "{{{}}}", bits)
    }
}