use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Eq, Hash)]
pub struct IPAddressV6(pub u128);

impl Display for IPAddressV6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let bytes = self.get_bytes();
        write!(
            f,
            "{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}",
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15],
        )
    }
}

impl PartialEq for IPAddressV6 {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

impl IPAddressV6 {
    pub fn get_bytes(&self) -> [u8; 16] {
        self.0.to_be_bytes()
    }
}
