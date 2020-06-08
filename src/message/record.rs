mod parameters;

use super::label::Label;
use crate::utils::{bytes_to_u16, bytes_to_u32, u16_to_bytes, u32_to_bytes};
pub use parameters::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct PartialRecord {
    pub label: String,
    pub rrtype: RRType,
    pub class: Class,
}

impl PartialRecord {
    pub fn from_offset(bytes: &[u8], mut offset: usize) -> (PartialRecord, usize) {
        let label = Label::from_offset(bytes, offset);

        match label.len() {
            1 => offset += 1,
            len => offset += len + 1,
        };

        let rrtype = RRType::from(bytes_to_u16(&bytes[offset..offset + 2]));
        let class = match rrtype {
            RRType::OPT => Class::NONE,
            _ => Class::from(bytes_to_u16(&bytes[offset + 2..offset + 4])),
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

        bytes.extend(u16_to_bytes(self.rrtype as u16));
        bytes.extend(u16_to_bytes(self.class as u16));

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

        let ttl = bytes_to_u32(&bytes[offset..offset + 4]);
        let data_len = bytes_to_u16(&bytes[offset + 4..offset + 6]) as usize;

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

        bytes.extend(u32_to_bytes(self.ttl));
        bytes.extend(u16_to_bytes(self.data.len() as u16));
        bytes.extend(self.data.iter());

        bytes
    }
}

impl fmt::Display for ResourceRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bytes = format!("");
        for byte in self.data.iter() {
            bytes = format!("{}{:02X} ", bytes, byte);
        }

        write!(
            f,
            "{} {} {:?} {:?} {}",
            self.label, self.ttl, self.class, self.rrtype, bytes
        )
    }
}
