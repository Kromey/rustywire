mod label;
mod record;

use crate::utils::bytes_to_u16;
use record::{PartialRecord, ResourceRecord};

#[derive(Debug)]
pub struct Message {
    id: u16,
    flags: u16,
    queries: Vec<PartialRecord>,
    answers: Vec<ResourceRecord>,
    authorities: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>,
    is_edns: bool,
}

impl From<Vec<u8>> for Message {
    fn from(bytes: Vec<u8>) -> Message {
        assert!(bytes.len() >= 12);

        let id = bytes_to_u16(&bytes[0..2]);
        let flags = bytes_to_u16(&bytes[2..4]);

        let queries = bytes_to_u16(&bytes[4..6]);
        let answers = bytes_to_u16(&bytes[6..8]);
        let authorities = bytes_to_u16(&bytes[8..10]);
        let additional = bytes_to_u16(&bytes[10..12]);

        let mut offset = 12;

        let mut msg = Message {
            id,
            flags,
            queries: Vec::with_capacity(queries as usize),
            answers: Vec::with_capacity(answers as usize),
            authorities: Vec::with_capacity(authorities as usize),
            additional: Vec::with_capacity(additional as usize),
            is_edns: false,
        };

        for _ in 0..queries {
            let (query, new_offset) = PartialRecord::from_offset(&bytes, offset);
            offset = new_offset;
            println!("{}", query);

            msg.queries.push(query);
        }
        for _ in 0..additional {
            let (record, new_offset) = ResourceRecord::from_offset(&bytes, offset);
            offset = new_offset;
            println!("{}", record);
            if let record::RRType::OPT = record.rrtype {
                msg.is_edns = true;
            };

            msg.additional.push(record);
        }

        msg
    }
}
