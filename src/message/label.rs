use std::str;

#[derive(Debug)]
pub struct Label {}

impl Label {
    pub fn from_offset(bytes: &[u8], mut offset: usize) -> (String, usize) {
        let mut label = String::new();

        if bytes[offset] == 0 {
            return (String::from("."), offset + 1);
        }

        while bytes[offset] > 0 {
            let len = bytes[offset] as usize;

            match len >> 6 {
                0 => {
                    offset += 1;
                    label.push_str(str::from_utf8(&bytes[offset..(offset + len)]).unwrap());
                    label.push('.');

                    offset += len;
                }
                3 => {
                    let mut pointer = (len << 8) | (bytes[offset + 1] as usize);
                    pointer &= 0x3FFF;

                    let (sub_label, _) = Label::from_offset(&bytes, pointer);
                    label.push_str(&sub_label);

                    return (label, offset + 2);
                }
                x => panic!("Invalid label type: 0b{:2b}", x),
            };
        }

        (label, offset + 1)
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
