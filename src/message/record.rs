mod parameters;

pub use parameters::{Class, RRType};
use std::fmt;
use super::label::Label;
use super::OffsetBytes;
use crate::utils::{bytes_to_u16, bytes_to_u32};

#[derive(Debug)]
pub struct PartialRecord<'a> {
    pub label: Label<'a>,
    pub rrtype: RRType,
    pub class: Class,
}

impl PartialRecord<'_> {
    pub fn len(&self) -> usize {
        self.label.len() + 4
    }
}

impl<'a> From<OffsetBytes<'a>> for PartialRecord<'a> {
    fn from(data: OffsetBytes<'a>) -> PartialRecord<'a> {
        let label = Label::from(OffsetBytes { ..data });

        let offset = data.offset + label.bytes;
        let rrtype = RRType::from(bytes_to_u16(&data.bytes[offset..offset+2]));
        let class = match rrtype {
            RRType::OPT => Class::NONE,
            _ => Class::from(bytes_to_u16(&data.bytes[offset+2..offset+4])),
        };

        PartialRecord {
            label,
            rrtype,
            class,
        }
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

impl ResourceRecord<'_> {
    pub fn len(&self) -> usize {
        self.label.len() + 10 + self.data.len()
    }
}

impl<'a> From<OffsetBytes<'a>> for ResourceRecord<'a> {
    fn from(data: OffsetBytes<'a>) -> ResourceRecord<'a> {
        let partial = PartialRecord::from(OffsetBytes { ..data });

        let mut offset = data.offset + partial.label.bytes + 4;
        let ttl = bytes_to_u32(&data.bytes[offset..offset+4]);
        let data_len = bytes_to_u16(&data.bytes[offset+4..offset+6]) as usize;

        offset += 6;

        ResourceRecord {
            label: partial.label,
            rrtype: partial.rrtype,
            class: partial.class,
            ttl,
            data: &data.bytes[offset..offset+data_len],
        }
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
