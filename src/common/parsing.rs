pub type U1 = u8;
pub type U2 = u8;
pub type U3 = u8;
pub type U4 = u8;
pub type U13 = u16;
pub type U24 = u32;

pub fn read_u8(buffer: &mut &[u8]) -> Option<u8> {
    let b = *(*buffer).get(0)?;
    *buffer = &(*buffer)[1..];
    Some(b)
}

pub fn read_u16(buffer: &mut &[u8]) -> Option<u16> {
    let [b0, b1] = read_array(buffer)?;
    Some(u16::from_be_bytes([b0, b1]))
}

pub fn read_u24(buffer: &mut &[u8]) -> Option<U24> {
    let [b0, b1, b2] = read_array(buffer)?;
    return Some(u32::from_be_bytes([0, b0, b1, b2]));
}

pub fn read_u32(buffer: &mut &[u8]) -> Option<u32> {
    let [b0, b1, b2, b3] = read_array(buffer)?;
    return Some(u32::from_be_bytes([b0, b1, b2, b3]));
}

pub fn read_array<const LEN: usize>(buffer: &mut &[u8]) -> Option<[u8; LEN]> {
    let mut arr = [0u8; LEN];
    let slice = buffer.get(..LEN)?;
    arr.copy_from_slice(slice);
    *buffer = &(*buffer)[LEN..];
    Some(arr)
}

pub fn read_vec(buffer: &mut &[u8], len: usize) -> Option<Vec<u8>> {
    let mut to_read = len;
    let readable_length = buffer.len();
    if len > readable_length {
        to_read = readable_length;
    }

    let v = Vec::from(buffer.get(..to_read)?);
    *buffer = &(*buffer)[to_read..];
    Some(v)
}
