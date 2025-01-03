use eyre::Context;

use crate::common::parsing::{U1, U2, U3};
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
    pub fn parse(num: u8) -> eyre::Result<TypeOfService> {
        Ok(TypeOfService {
            precedence: Precedence::parse(num).wrap_err("parsing precedence")?,
            delay: Delay::parse(num).wrap_err("parsing delay")?,
            throughput: Throughput::parse(num).wrap_err("parsing throughput")?,
            reliability: Reliability::parse(num).wrap_err("parsing reliability")?,
            reserved: num & 0b00000011,
        })
    }

    pub fn serialize(&self) -> u8 {
        let mut num = self.precedence.serialize() << 5;
        num |= self.delay.serialize() << 4;
        num |= self.throughput.serialize() << 3;
        num |= self.reliability.serialize() << 2;
        num
    }
}

impl Default for TypeOfService {
    fn default() -> Self {
        TypeOfService {
            precedence: Precedence::Routine,
            delay: Delay::Normal,
            throughput: Throughput::Normal,
            reliability: Reliability::Normal,
            reserved: 0,
        }
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
    fn parse(num: U3) -> eyre::Result<Precedence> {
        let val = (num & 0b11100000) >> 5;
        Ok(match val {
            0b111 => Precedence::NetworkControl,
            0b110 => Precedence::InternetworkControl,
            0b101 => Precedence::CriticECP,
            0b100 => Precedence::FlashOverride,
            0b011 => Precedence::Flash,
            0b010 => Precedence::Immediate,
            0b001 => Precedence::Priority,
            0b000 => Precedence::Routine,
            v => {
                eyre::bail!("Invalid precedence {}", v);
            }
        })
    }

    fn serialize(&self) -> U3 {
        match self {
            Precedence::NetworkControl => 0b111,
            Precedence::InternetworkControl => 0b110,
            Precedence::CriticECP => 0b101,
            Precedence::FlashOverride => 0b100,
            Precedence::Flash => 0b011,
            Precedence::Immediate => 0b010,
            Precedence::Priority => 0b001,
            Precedence::Routine => 0b000,
        }
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
    fn parse(num: u8) -> eyre::Result<Delay> {
        let val = (num & 0b00010000) >> 4;
        Ok(match val {
            0b0 => Delay::Normal,
            0b1 => Delay::Low,
            v => {
                eyre::bail!("Invalid delay: {}", v);
            }
        })
    }

    fn serialize(&self) -> U1 {
        match self {
            Delay::Normal => 0b0,
            Delay::Low => 0b1,
        }
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
    fn parse(num: u8) -> eyre::Result<Throughput> {
        let val = (num & 0b00001000) >> 3;
        Ok(match val {
            0b0 => Throughput::Normal,
            0b1 => Throughput::High,
            v => {
                eyre::bail!("Invalid throughput: {}", v);
            }
        })
    }

    fn serialize(&self) -> U1 {
        match self {
            Throughput::High => 0b1,
            Throughput::Normal => 0b0,
        }
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
    fn parse(num: u8) -> eyre::Result<Reliability> {
        let val = (num & 0b00000100) >> 2;
        Ok(match val {
            0b0 => Reliability::Normal,
            0b1 => Reliability::High,
            v => {
                eyre::bail!("Invalid reliability: {}", v);
            }
        })
    }

    fn serialize(&self) -> U1 {
        match self {
            Reliability::High => 0b1,
            Reliability::Normal => 0b0,
        }
    }
}
