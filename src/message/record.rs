mod parameters;

pub use parameters::{Class, RRType};
use std::fmt;
use super::label::Label;
use crate::utils::{bytes_to_u16, bytes_to_u32};

#[derive(Debug)]
pub struct PartialRecord<'a> {
    pub label: Label<'a>,
    pub rrtype: RRType,
    pub class: Class,
}

impl<'a> PartialRecord<'a> {
    pub fn from_offset(bytes: &'a [u8], offset: usize) -> PartialRecord<'a> {
        let label = Label::from_offset(bytes, offset);

        let offset = offset + label.len();
        let rrtype = RRType::from(bytes_to_u16(&bytes[offset..offset+2]));
        let class = match rrtype {
            RRType::OPT => Class::NONE,
            _ => Class::from(bytes_to_u16(&bytes[offset+2..offset+4])),
        };

        PartialRecord {
            label,
            rrtype,
            class,
        }
    }

    pub fn len(&self) -> usize {
        self.label.len() + 4
    }
}

impl fmt::Display for PartialRecord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{:?}\t{:?}", self.label, self.class, self.rrtype)
    }
}

#[derive(Debug)]
pub struct ResourceRecord<'a> {
    pub label: Label<'a>,
    pub rrtype: RRType,
    pub class: Class,
    pub ttl: u32,
    pub data: &'a [u8],
}

impl<'a> ResourceRecord<'a> {
    pub fn from_offset(bytes: &'a [u8], offset: usize) -> ResourceRecord<'a> {
        let partial = PartialRecord::from_offset(bytes, offset);

        let mut offset = offset + partial.len();
        let ttl = bytes_to_u32(&bytes[offset..offset+4]);
        let data_len = bytes_to_u16(&bytes[offset+4..offset+6]) as usize;

        offset += 6;

        ResourceRecord {
            label: partial.label,
            rrtype: partial.rrtype,
            class: partial.class,
            ttl,
            data: &bytes[offset..offset+data_len],
        }
    }

    pub fn len(&self) -> usize {
        self.label.len() + 10 + self.data.len()
    }
}

impl fmt::Display for ResourceRecord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = format!("");
        for byte in self.data.iter() {
            bytes = format!("{}{:02X} ", bytes, byte);
        }

        write!(f, "{}\t{}\t{:?}\t{:?}\t{}", self.label, self.ttl, self.class, self.rrtype, bytes)
    }
}
