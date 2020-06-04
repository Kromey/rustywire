mod label;
mod record;

use crate::utils::bytes_to_u16;
use record::{PartialRecord, ResourceRecord};

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

        let counts = [
            bytes_to_u16(&bytes[4..]),
            bytes_to_u16(&bytes[6..]),
            bytes_to_u16(&bytes[8..]),
            bytes_to_u16(&bytes[10..]),
        ];

        let mut offset = 12;

        let mut queries = Vec::<PartialRecord>::with_capacity(counts[0] as usize);
        let mut records = [
            Vec::<ResourceRecord>::with_capacity(counts[1] as usize),
            Vec::<ResourceRecord>::with_capacity(counts[2] as usize),
            Vec::<ResourceRecord>::with_capacity(counts[3] as usize),
        ];
        for (i, count) in counts.iter().enumerate() {
            for _ in 0..*count {
                let data = OffsetBytes {
                    bytes: &bytes,
                    offset,
                };

                if i == 0 {
                    let query = PartialRecord::from(data);
                    println!("{}", query);

                    offset += query.len();

                    queries.push(query);
                } else {
                    let record = ResourceRecord::from(data);
                    println!("{}", record);

                    offset += record.len();

                    records[i-1].push(record);
                }
            }
        }

        Message {
            bytes,
        }
    }
}
