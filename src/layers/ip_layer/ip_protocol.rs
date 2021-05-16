use std::fmt::{Display, Formatter};
use std::fmt;

// The IP protocol as determined by https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml
#[derive(Clone, Debug)]
pub enum Protocol {
    HOPOPT,
    ICMP,
    IPv4,
    TCP,
    IPv6,
    Other(u8),
}

impl Protocol {
    pub fn parse(num: u8) -> Protocol {
        match num {
            0 => Protocol::HOPOPT,
            1 => Protocol::ICMP,
            4 => Protocol::IPv4,
            6 => Protocol::TCP,
            41 => Protocol::IPv6,
            v => Protocol::Other(v),
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::HOPOPT => write!(f, "(HOPOPT) IPv6 Hop-by-Hop Option"),
            Protocol::ICMP => write!(f, "(ICMP) Internet Control Message"),
            Protocol::IPv4 => write!(f, "(IPv4) IPv4 encapsulation"),
            Protocol::TCP => write!(f, "(TCP) Transmission Control"),
            Protocol::IPv6 => write!(f, "(IPv6) IPv6 encapsulation"),
            Protocol::Other(v) => write!(f, "Other ({})", v),
        }
    }
}
