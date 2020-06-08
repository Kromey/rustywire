pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    (bytes[0] as u16) << 8 | bytes[1] as u16
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    (bytes_to_u16(&bytes[0..2]) as u32) << 16 | bytes_to_u16(&bytes[2..4]) as u32
}

pub fn u16_to_bytes(val: u16) -> Vec<u8> {
    vec![(val >> 8) as u8, val as u8]
}

pub fn u32_to_bytes(val: u32) -> Vec<u8> {
    let mut bytes = u16_to_bytes((val >> 16) as u16);
    bytes.extend(u16_to_bytes(val as u16));

    bytes
}
