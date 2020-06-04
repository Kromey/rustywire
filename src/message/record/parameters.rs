#[derive(Debug)]
pub enum Class {
    IN = 1,
    CH = 3,
    HS = 4,
    NONE = 254,
    ANY = 255,
}

#[derive(Debug)]
pub enum RRType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    SRV = 33,
    OPT = 41,
    ANY = 255,
}

impl From<u16> for Class {
    fn from(class: u16) -> Class {
        match class {
            1 => Class::IN,
            3 => Class::CH,
            4 => Class::HS,
            254 => Class::NONE,
            255 => Class::ANY,
            e => unimplemented!("Class: {}", e),
        }
    }
}

impl From<u16> for RRType {
    fn from(class: u16) -> RRType {
        match class {
            1 => RRType::A,
            2 => RRType::NS,
            5 => RRType::CNAME,
            6 => RRType::SOA,
            15 => RRType::MX,
            16 => RRType::TXT,
            28 => RRType::AAAA,
            33 => RRType::SRV,
            41 => RRType::OPT,
            255 => RRType::ANY,
            e => unimplemented!("Resource Type: {}", e),
        }
    }
}
