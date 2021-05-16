use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct IPAddress(pub u32);

impl Display for IPAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let bytes = self.get_bytes();
        write!(f, "{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
    }
}

impl IPAddress {
    pub fn get_bytes(&self) -> [u8; 4] {
        let first = (self.0 >> 24) as u8;
        let second = (self.0 >> 16) as u8;
        let third = (self.0 >> 8) as u8;
        let fourth = self.0 as u8;
        return [first, second, third, fourth];
    }
}
