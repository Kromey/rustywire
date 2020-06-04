use std::str;
use super::OffsetBytes;

#[derive(Debug)]
pub struct Label<'a> {
    pub label: Vec<&'a str>,
    pub bytes: usize,
}

impl<'a> From<OffsetBytes<'a>> for Label<'a> {
    fn from(msg: OffsetBytes<'a>) -> Label<'a> {
        let mut offset = msg.offset;
        let mut bytes = 0;
        let mut label = Vec::new();

        while msg.bytes[offset] > 0 {
            let length = msg.bytes[offset] as usize;
            offset += 1;
            label.push(str::from_utf8(&msg.bytes[offset..(offset+length)]).unwrap());

            offset += length;
            bytes += length + 1;
        }
        // Account for the final byte we read
        bytes += 1;

        Label {
            label,
            bytes,
        }
    }
}
