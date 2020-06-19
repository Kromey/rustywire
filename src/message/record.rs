mod parameters;

use bytes::Bytes;
pub use parameters::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct PartialRecord {
    pub label: String,
    pub rrtype: RRType,
    pub class: Class,
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
    pub data: Bytes,
}

impl ResourceRecord {
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
