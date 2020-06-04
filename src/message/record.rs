mod parameters;

pub use parameters::{Class, RRType};
use std::fmt;
use super::label::Label;
use super::OffsetBytes;
use crate::utils::bytes_to_u16;

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
        let class = Class::from(bytes_to_u16(&data.bytes[offset+2..offset+4]));

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