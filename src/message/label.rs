use std::{fmt, str};
use super::OffsetBytes;

#[derive(Debug)]
pub struct Label<'a> {
    pub label: Vec<&'a str>,
    pub bytes: usize,
}

impl Label<'_> {
    pub fn len(&self) -> usize {
        self.bytes
    }
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

impl fmt::Display for Label<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.len() == 1 {
            write!(f, ".")
        } else {
            let mut output = format!("");
            for part in self.label.iter() {
                output = format!("{}{}.", output, part);
            }
            write!(f, "{}", output)
        }
    }
}
