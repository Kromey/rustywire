#[macro_export]
macro_rules! bytes_to {
    ($T:ty, $bytes:expr) => {{
        let mut _start = 0;
        bytes_to!($T, $bytes, _start)
    }};
    ($T:ty, $bytes:expr, $start:ident) => {{
        use std::convert::TryInto;
        let size = std::mem::size_of::<$T>();
        let (bytes, _) = $bytes[$start..].split_at(size);
        $start += size;
        <$T>::from_be_bytes(bytes.try_into().unwrap())
    }};
}

#[macro_export]
macro_rules! int_to_bytes {
    ($val:expr) => {
        $val.to_be_bytes().to_vec()
    };
}

#[macro_export]
macro_rules! decompose {
    ($bytes:expr, $($T:ident),+) => {{
        let mut start = 0;
        let val = (
            $(bytes_to!($T, $bytes, start),)+
        );

        assert_eq!(start, $bytes.len());

        val
    }};
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
