use std::fmt;
use std::fmt::{Display, Formatter};

// The IP protocol as determined by https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml
#[derive(Clone, Debug)]
pub enum Protocol {
    HOPOPT,
    ICMP,
    IPv4,
    TCP,
    UDP,
    IPv6,
    IPv6ICMP,
    Other(u8),
}

impl Protocol {
    pub fn parse(num: u8) -> Protocol {
        match num {
            0 => Protocol::HOPOPT,
            1 => Protocol::ICMP,
            4 => Protocol::IPv4,
            6 => Protocol::TCP,
            17 => Protocol::UDP,
            41 => Protocol::IPv6,
            58 => Protocol::IPv6ICMP,
            v => Protocol::Other(v),
        }
    }

    pub fn serialize(&self) -> u8 {
        match self {
            Protocol::HOPOPT => 0,
            Protocol::ICMP => 1,
            Protocol::IPv4 => 4,
            Protocol::TCP => 6,
            Protocol::UDP => 17,
            Protocol::IPv6 => 41,
            Protocol::IPv6ICMP => 58,
            Protocol::Other(v) => v.clone(),
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
            Protocol::UDP => write!(f, "(UDP) User Datagram"),
            Protocol::IPv6 => write!(f, "(IPv6) IPv6 encapsulation"),
            Protocol::IPv6ICMP => {
                write!(f, "(IPv6 ICMP) Internet Control Message Protocol over IPv6")
            }
            Protocol::Other(v) => write!(f, "Other ({})", v),
        }
    }
}
