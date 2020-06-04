mod parameters;

pub use parameters::{Class, RRType};
use super::label::Label;
use super::OffsetBytes;
use crate::utils::bytes_to_u16;

#[derive(Debug)]
pub struct PartialRecord<'a> {
    pub label: Label<'a>,
    pub rrtype: RRType,
    pub class: Class,
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
