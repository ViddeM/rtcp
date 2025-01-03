use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Hash)]
pub struct IPAddressV4(pub u32);

impl Display for IPAddressV4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let bytes = self.get_bytes();
        write!(f, "{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
    }
}

impl PartialEq for IPAddressV4 {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

impl IPAddressV4 {
    pub fn get_bytes(&self) -> [u8; 4] {
        let first = (self.0 >> 24) as u8;
        let second = (self.0 >> 16) as u8;
        let third = (self.0 >> 8) as u8;
        let fourth = self.0 as u8;
        return [first, second, third, fourth];
    }
}
