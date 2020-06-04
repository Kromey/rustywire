mod label;
mod record;

use crate::utils::bytes_to_u16;
use record::PartialRecord;

struct OffsetBytes<'a> {
    pub bytes: &'a Vec<u8>,
    pub offset: usize,
}

#[derive(Debug)]
pub struct Message {
    bytes: Vec<u8>,
}

impl From<Vec<u8>> for Message {
    fn from(bytes: Vec<u8>) -> Message {
        assert!(bytes.len() >= 12);

        let qry_count = bytes_to_u16(&bytes[4..]);
        let ans_count = bytes_to_u16(&bytes[6..]);
        let auth_count = bytes_to_u16(&bytes[8..]);
        let addl_count = bytes_to_u16(&bytes[10..]);

        let mut offset = 12;
        for _ in 0..qry_count {
            let data = OffsetBytes {
                bytes: &bytes,
                offset,
            };
            let query = PartialRecord::from(data);
            println!("{}", query);

            offset += query.len();
        }

        Message {
            bytes,
        }
    }
}
