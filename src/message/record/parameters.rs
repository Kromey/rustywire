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

/// DNS Header Flags
#[derive(Debug)]
pub enum Flags {
    /// Query/Response: Set if a response, clear if a query
    QR = 0x8000,
    /// Authoritative Answer: Responding name server is authoritative (response)
    AA = 0x0400,
    /// Truncation: Indicated this response was truncated (response)
    TC = 0x0200,
    /// Recursion Desired: Requests the name server to recursively resolve the query (query)
    RD = 0x0100,
    /// Recursion Available: Indicates support for recursively resolving requests (response)
    RA = 0x0080,
    /// Authenticated Data: Requests DNSSEC validation, or indicates DNSSEC validation was
    /// successful
    AD = 0x0020,
    /// Checking Disabled: Indicates the client does not wish the server to validate DNSSEC
    CD = 0x0010,
}

/// DNS Header OpCode
#[derive(Debug)]
pub enum OpCode {
    /// Query (RFC1035)
    Query = 0,
    /// Server Status (RFC1035)
    Status = 2,
    /// Notify (RFC1996)
    Notify = 4,
    /// Update (RFC2136)
    Update = 5,
    /// DNS Stateful Operations (RFC8490)
    DSO = 6,
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
