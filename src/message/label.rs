use std::{fmt, str};

#[derive(Debug)]
pub struct Label<'a> {
    pub label: Vec<&'a str>,
    pub bytes: usize,
}

impl<'a> Label<'a> {
    pub fn from_offset(bytes: &'a [u8], offset: usize) -> Label<'a> {
        let mut offset = offset;
        let mut length = 0;
        let mut label = Vec::new();

        while bytes[offset] > 0 {
            let len = bytes[offset] as usize;
            offset += 1;
            label.push(str::from_utf8(&bytes[offset..(offset+len)]).unwrap());

            offset += len;
            length += len + 1;
        }
        // Account for the final byte we read
        length += 1;

        Label {
            label,
            bytes: length,
        }
    }

    pub fn len(&self) -> usize {
        self.bytes
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
