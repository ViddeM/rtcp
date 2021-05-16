use crate::common::parsing::U2;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct TypeOfService {
    pub precedence: Precedence,
    pub delay: Delay,
    pub throughput: Throughput,
    pub reliability: Reliability,
    pub reserved: U2,
}

impl Display for TypeOfService {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{
    precedence: {},
    delay: {},
    throughput: {},
    reliability: {},
    reserved: {:x},
}}",
            self.precedence, self.delay, self.throughput, self.reliability, self.reserved
        )
    }
}

impl TypeOfService {
    pub fn parse(num: u8) -> Option<TypeOfService> {
        Some(TypeOfService {
            precedence: Precedence::parse(num)?,
            delay: Delay::parse(num)?,
            throughput: Throughput::parse(num)?,
            reliability: Reliability::parse(num)?,
            reserved: num & 0b00000011,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Precedence {
    NetworkControl,
    InternetworkControl,
    CriticECP,
    FlashOverride,
    Flash,
    Immediate,
    Priority,
    Routine,
}

impl Display for Precedence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Precedence::NetworkControl => write!(f, "Network Control"),
            Precedence::InternetworkControl => write!(f, "Internetwork Control"),
            Precedence::CriticECP => write!(f, "CRITIC/ECP"),
            Precedence::FlashOverride => write!(f, "Flash Override"),
            Precedence::Flash => write!(f, "Flash"),
            Precedence::Immediate => write!(f, "Immediate"),
            Precedence::Priority => write!(f, "Priority"),
            Precedence::Routine => write!(f, "Routine"),
        }
    }
}

impl Precedence {
    fn parse(num: u8) -> Option<Precedence> {
        let val = (num & 0b11100000) >> 5;
        Some(match val {
            0b111 => Precedence::NetworkControl,
            0b110 => Precedence::InternetworkControl,
            0b101 => Precedence::CriticECP,
            0b100 => Precedence::FlashOverride,
            0b011 => Precedence::Flash,
            0b010 => Precedence::Immediate,
            0b001 => Precedence::Priority,
            0b000 => Precedence::Routine,
            v => {
                eprintln!("Invalid precedence {}", v);
                return None;
            }
        })
    }
}

#[derive(Clone, Debug)]
pub enum Delay {
    Normal,
    Low,
}

impl Display for Delay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Delay::Normal => write!(f, "Normal"),
            Delay::Low => write!(f, "Low"),
        }
    }
}

impl Delay {
    fn parse(num: u8) -> Option<Delay> {
        let val = (num & 0b00010000) >> 4;
        Some(match val {
            0b0 => Delay::Normal,
            0b1 => Delay::Low,
            v => {
                eprintln!("Invalid delay: {}", v);
                return None;
            }
        })
    }
}

#[derive(Clone, Debug)]
pub enum Throughput {
    Normal,
    High,
}

impl Display for Throughput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Throughput::Normal => write!(f, "Normal"),
            Throughput::High => write!(f, "High"),
        }
    }
}

impl Throughput {
    fn parse(num: u8) -> Option<Throughput> {
        let val = (num & 0b00001000) >> 3;
        Some(match val {
            0b0 => Throughput::Normal,
            0b1 => Throughput::High,
            v => {
                eprintln!("Invalid throughput: {}", v);
                return None;
            }
        })
    }
}

#[derive(Clone, Debug)]
pub enum Reliability {
    Normal,
    High,
}

impl Display for Reliability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Reliability::Normal => write!(f, "Normal"),
            Reliability::High => write!(f, "High"),
        }
    }
}

impl Reliability {
    fn parse(num: u8) -> Option<Reliability> {
        let val = (num & 0b00000100) >> 2;
        Some(match val {
            0b0 => Reliability::Normal,
            0b1 => Reliability::High,
            v => {
                eprintln!("Invalid reliability: {}", v);
                return None;
            }
        })
    }
}
