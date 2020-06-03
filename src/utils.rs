pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    (bytes[0] as u16) << 8 | bytes[1] as u16
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    (bytes_to_u16(&bytes[0..2]) as u32) << 16 | bytes_to_u16(&bytes[2..4]) as u32
}

