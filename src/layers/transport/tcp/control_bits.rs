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

impl Display for ControlBits {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut bits = String::from(
                (if self.urg { "URG\n" } else { ""}).to_owned() +
                (if self.ack { "ACK\n" } else { ""}) +
                (if self.psh { "PSH\n" } else { ""}) +
                (if self.rst { "RST\n" } else { ""}) +
                (if self.syn { "SYN\n" } else { ""}) +
                (if self.fin { "FIN\n" } else { ""})
        );

        if bits.ends_with("\n") {
            bits.remove(bits.len() - 1);
        }

        write!(f, "Control Bits: {{
    {}
}}", bits)
    }
}