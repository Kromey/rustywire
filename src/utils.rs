use std::convert::TryInto;

pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    let (bytes, _) = bytes.split_at(std::mem::size_of::<u16>());

    u16::from_be_bytes(bytes.try_into().unwrap())
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let (bytes, _) = bytes.split_at(std::mem::size_of::<u32>());

    u32::from_be_bytes(bytes.try_into().unwrap())
}

pub fn u16_to_bytes(val: u16) -> Vec<u8> {
    val.to_be_bytes()[..].into()
}

pub fn u32_to_bytes(val: u32) -> Vec<u8> {
    val.to_be_bytes()[..].into()
}

const HEX_LINE_SIZE: usize = 12;

pub fn dump_hex(bytes: &[u8]) {
    let number_of_bytes = bytes.len();

    for offset in (0..number_of_bytes).step_by(HEX_LINE_SIZE) {
        print!("{:04} ", offset);
        for byte in bytes[offset..(offset + HEX_LINE_SIZE).min(number_of_bytes)].iter() {
            print!("{:02X} ", byte);
        }
        println!("");
    }
    println!("");
}
