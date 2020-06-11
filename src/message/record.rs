mod parameters;

use super::label::Label;
use crate::{bytes_to, decompose, int_to_bytes};
pub use parameters::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct PartialRecord {
    pub label: String,
    pub rrtype: RRType,
    pub class: Class,
}

impl PartialRecord {
    pub fn from_offset(bytes: &[u8], offset: usize) -> (PartialRecord, usize) {
        let (label, mut offset) = Label::from_offset(bytes, offset);

        let (rrtype, class) = decompose!(bytes[offset..offset + 4], u16, u16);

        let rrtype = RRType::from(rrtype);
        let class = match rrtype {
            RRType::OPT => Class::NONE,
            _ => Class::from(class),
        };
        offset += 4;

        (
            PartialRecord {
                label,
                rrtype,
                class,
            },
            offset,
        )
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.label.len() + 5);

        bytes.extend(Label::as_bytes(&self.label));

        bytes.extend(int_to_bytes!(self.rrtype as u16));
        bytes.extend(int_to_bytes!(self.class as u16));

        bytes
    }
}

impl fmt::Display for PartialRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {:?}", self.label, self.class, self.rrtype)
    }
}

#[derive(Clone, Debug)]
pub struct ResourceRecord {
    pub label: String,
    pub rrtype: RRType,
    pub class: Class,
    pub ttl: u32,
    pub data: Vec<u8>,
}

impl ResourceRecord {
    pub fn from_offset(bytes: &[u8], offset: usize) -> (ResourceRecord, usize) {
        let (partial, mut offset) = PartialRecord::from_offset(bytes, offset);

        let (ttl, data_len) = decompose!(bytes[offset..offset + 6], u32, u16);
        let data_len = data_len as usize;

        offset += 6;

        let data = (&bytes[offset..offset + data_len]).to_vec();
        offset += data_len;

        (
            ResourceRecord {
                label: partial.label,
                rrtype: partial.rrtype,
                class: partial.class,
                ttl,
                data,
            },
            offset,
        )
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = PartialRecord {
            label: self.label.clone(),
            rrtype: self.rrtype,
            class: self.class,
        }
        .as_bytes();

        bytes.extend(int_to_bytes!(self.ttl));
        bytes.extend(int_to_bytes!(self.data.len() as u16));
        bytes.extend(self.data.iter());

        bytes
    }

    fn format_rdata(&self) -> String {
        match self.rrtype {
            RRType::A => format!(
                "{}.{}.{}.{}",
                self.data[0], self.data[1], self.data[2], self.data[3]
            ),
            _ => {
                let mut bytes = format!("");
                for byte in self.data.iter() {
                    bytes = format!("{}{:02X} ", bytes, byte);
                }

                bytes
            }
        }
    }
}

impl fmt::Display for ResourceRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {:?} {:?} {}",
            self.label,
            self.ttl,
            self.class,
            self.rrtype,
            self.format_rdata()
        )
    }
}
