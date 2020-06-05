mod label;
mod record;

use crate::utils::bytes_to_u16;
use record::{PartialRecord, ResourceRecord};

#[derive(Debug)]
pub struct Message<'a> {
    bytes: Vec<u8>,
    queries: Vec<PartialRecord<'a>>,
    answers: Vec<ResourceRecord<'a>>,
    authorities: Vec<ResourceRecord<'a>>,
    additional: Vec<ResourceRecord<'a>>,
}

impl<'a> From<Vec<u8>> for Message<'a> {
    fn from(bytes: Vec<u8>) -> Message<'a> {
        assert!(bytes.len() >= 12);

        let counts = [
            bytes_to_u16(&bytes[4..]),
            bytes_to_u16(&bytes[6..]),
            bytes_to_u16(&bytes[8..]),
            bytes_to_u16(&bytes[10..]),
        ];

        let mut offset = 12;

        let mut msg = Message {
            bytes,
            queries: Vec::<PartialRecord>::with_capacity(counts[0] as usize),
            answers: Vec::<ResourceRecord>::with_capacity(counts[1] as usize),
            authorities: Vec::<ResourceRecord>::with_capacity(counts[2] as usize),
            additional: Vec::<ResourceRecord>::with_capacity(counts[3] as usize),
        };

        for _ in 0..counts[0] {
            let query = PartialRecord::from_offset(&msg.bytes, offset);
            println!("{}", query);

            offset += query.len();

            msg.queries.push(query);
        }
        for _ in 0..counts[3] {
            let record = ResourceRecord::from_offset(&msg.bytes, offset);
            println!("{}", record);

            offset += record.len();

            msg.additional.push(record);
        }

        msg
    }
}
