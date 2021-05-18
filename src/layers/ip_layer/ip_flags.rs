use crate::common::parsing::{U1, U3};
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Flags {
    reserved: U1,
    df: DF,
    mf: MF
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Flags {{
    reserved: {},
    DF: {},
    MF: {},
}}", self.reserved, self.df, self.mf)
    }
}

impl Flags {
    pub fn parse(num: U3) -> Option<Flags> {
        Some(Flags {
            reserved: num & 0b100 >> 2,
            df: DF::parse(num)?,
            mf: MF::parse(num)?,
        })
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            reserved: 0,
            df: DF::DontFragment,
            mf: MF::LastFragment,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DF {
    MayFragment,
    DontFragment
}

impl Display for DF {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DF::MayFragment => write!(f, "May fragment"),
            DF::DontFragment => write!(f, "Don't fragment"),
        }
    }
}

impl DF {
    fn parse(num: U3) -> Option<DF> {
        Some(match (num & 0b010) >> 1 {
            0 => DF::MayFragment,
            1 => DF::DontFragment,
            v => {
                eprintln!("Invalid DF flag bit {}", v);
                return None
            },
        })
    }
}

#[derive(Clone, Debug)]
pub enum MF {
    LastFragment,
    MoreFragments,
}

impl Display for MF {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MF::LastFragment => write!(f, "Last fragment"),
            MF::MoreFragments => write!(f, "More fragments")
        }
    }
}

impl MF {
    fn parse(num: U3) -> Option<MF> {
        Some(match num & 0b001 {
            0 => MF::LastFragment,
            1 => MF::MoreFragments,
            v => {
                eprintln!("Invalid MF flag bit {}", v);
                return None
            },
        })
    }
}