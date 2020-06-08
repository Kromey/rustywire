use std::str;

#[derive(Debug)]
pub struct Label {}

impl Label {
    pub fn from_offset(bytes: &[u8], offset: usize) -> String {
        let mut offset = offset;
        let mut label = String::new();

        if bytes[offset] == 0 {
            return String::from(".");
        }

        while bytes[offset] > 0 {
            let len = bytes[offset] as usize;
            offset += 1;
            label.push_str(str::from_utf8(&bytes[offset..(offset + len)]).unwrap());
            label.push('.');

            offset += len;
        }

        label
    }

    pub fn as_bytes(label: &str) -> Vec<u8> {
        if label == "." {
            return vec![0];
        }

        let mut bytes = Vec::new();
        for l in label.split('.') {
            bytes.push(l.len() as u8);
            bytes.extend(l.as_bytes());
        }

        bytes
    }
}
