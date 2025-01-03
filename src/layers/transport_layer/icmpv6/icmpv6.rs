use std::fmt::Display;

use colored::Colorize;
use eyre::ContextCompat;

use crate::common::{
    parsing::{read_u16, read_u8},
    proto::Proto,
};

use super::icmpv6_type::ICMPv6Type;

#[derive(Debug, Clone)]
pub struct ICMPv6 {
    pub message_type: ICMPv6Type,
    pub checksum: u16,
}

impl Display for ICMPv6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ICMPv6 {{
    message_type: {},
    checksum: {:x}
        ",
            self.message_type, self.checksum
        )
    }
}

impl Proto for ICMPv6 {
    fn to_short_string(&self) -> String {
        format!(
            "{} ({}) {}",
            "ICMP".blue(),
            "v6".purple(),
            self.message_type.to_short_string()
        )
    }

    fn parse(buf: &mut &[u8]) -> eyre::Result<Self> {
        let m_type = read_u8(buf).wrap_err("reading message type")?;
        let code = read_u8(buf).wrap_err("reading code")?;
        let checksum = read_u16(buf).wrap_err("readng checksum")?;

        let message_type = ICMPv6Type::parse(m_type, code, buf).wrap_err("reading message type")?;

        Ok(Self {
            message_type,
            checksum,
        })
    }
}
