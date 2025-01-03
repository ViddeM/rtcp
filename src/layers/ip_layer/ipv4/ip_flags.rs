use eyre::Context;

use crate::common::parsing::{U1, U3};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Flags {
    reserved: U1,
    df: DF,
    mf: MF,
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Flags {{
    reserved: {},
    DF: {},
    MF: {},
}}",
            self.reserved, self.df, self.mf
        )
    }
}

impl Flags {
    pub fn parse(num: U3) -> eyre::Result<Flags> {
        Ok(Flags {
            reserved: num & 0b100 >> 2,
            df: DF::parse(num).wrap_err("parsing DF")?,
            mf: MF::parse(num).wrap_err("parsing MF")?,
        })
    }

    pub fn serialize(&self) -> U3 {
        0 | (self.df.serialize() << 1) | self.mf.serialize()
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
    DontFragment,
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
    fn parse(num: U1) -> eyre::Result<DF> {
        Ok(match (num & 0b010) >> 1 {
            0 => DF::MayFragment,
            1 => DF::DontFragment,
            v => {
                eyre::bail!("Invalid DF flag bit {}", v);
            }
        })
    }

    fn serialize(&self) -> U1 {
        match self {
            DF::MayFragment => 0,
            DF::DontFragment => 1,
        }
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
            MF::MoreFragments => write!(f, "More fragments"),
        }
    }
}

impl MF {
    fn parse(num: U3) -> eyre::Result<MF> {
        Ok(match num & 0b001 {
            0 => MF::LastFragment,
            1 => MF::MoreFragments,
            v => {
                eyre::bail!("Invalid MF flag bit {}", v);
            }
        })
    }

    fn serialize(&self) -> U1 {
        match self {
            MF::LastFragment => 0,
            MF::MoreFragments => 1,
        }
    }
}

