mod label;
mod record;

use crate::utils::{bytes_to_u16, u16_to_bytes};
pub use record::{Flags, OpCode, RCode};
use record::{PartialRecord, ResourceRecord};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Message {
    id: u16,
    flags: u16,
    queries: Vec<PartialRecord>,
    answers: Vec<ResourceRecord>,
    authorities: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>,
    is_edns: bool,
}

impl Message {
    pub fn into_response(self) -> Message {
        // Stash a few values we'll need to copy to our response
        let opcode = self.get_opcode();
        let rd = self.get_flag(Flags::RD);

        let mut resp = Message {
            flags: 0,
            answers: Vec::new(),
            authorities: Vec::new(),
            additional: Vec::new(),
            is_edns: false,
            ..self
        };

        resp.set_flag(Flags::QR);
        if rd {
            resp.set_flag(Flags::RD);
        }
        resp.set_opcode(opcode);

        resp
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        let flag = flag as u16;

        self.flags & flag > 0
    }

    pub fn set_flag(&mut self, flag: Flags) {
        self.flags |= flag as u16;
    }

    pub fn get_opcode(&self) -> OpCode {
        let code = (self.flags >> 11) & 0x0F;

        match code {
            0 => OpCode::Query,
            2 => OpCode::Status,
            4 => OpCode::Notify,
            5 => OpCode::Update,
            6 => OpCode::DSO,
            _ => panic!("Unknown OpCode"),
        }
    }

    pub fn set_opcode(&mut self, code: OpCode) {
        // Mask off any existing OpCode
        self.flags &= !(0x0F << 11);
        // Now we can safely set our OpCode
        self.flags |= (code as u16) << 11;
    }

    pub fn get_rcode(&self) -> RCode {
        let code = self.flags & 0x0F;

        match code {
            0 => RCode::NoError,
            1 => RCode::FormErr,
            2 => RCode::ServFail,
            3 => RCode::NXDomain,
            4 => RCode::NotImp,
            5 => RCode::Refused,
            _ => panic!("Uknown RCode"),
        }
    }

    pub fn set_rcode(&mut self, code: RCode) {
        // Mask off any existing RCode
        self.flags &= !(0x0F);
        // Now we can safetly set our RCode
        self.flags |= code as u16;
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(512);

        bytes.extend(u16_to_bytes(self.id));
        bytes.extend(u16_to_bytes(self.flags));

        bytes.extend(u16_to_bytes(self.queries.len() as u16));
        bytes.extend(u16_to_bytes(self.answers.len() as u16));
        bytes.extend(u16_to_bytes(self.authorities.len() as u16));
        bytes.extend(u16_to_bytes(self.additional.len() as u16));

        for q in self.queries.iter() {
            bytes.extend(q.as_bytes());
        }
        for records in vec![&self.answers, &self.authorities, &self.additional] {
            for record in records.iter() {
                bytes.extend(record.as_bytes());
            }
        }

        bytes
    }
}

impl From<Vec<u8>> for Message {
    fn from(bytes: Vec<u8>) -> Message {
        assert!(bytes.len() >= 12);

        let id = bytes_to_u16(&bytes[0..2]);
        let flags = bytes_to_u16(&bytes[2..4]);

        let queries = bytes_to_u16(&bytes[4..6]);
        let answers = bytes_to_u16(&bytes[6..8]);
        let authorities = bytes_to_u16(&bytes[8..10]);
        let additional = bytes_to_u16(&bytes[10..12]);

        let mut offset = 12;

        let mut msg = Message {
            id,
            flags,
            queries: Vec::with_capacity(queries as usize),
            answers: Vec::with_capacity(answers as usize),
            authorities: Vec::with_capacity(authorities as usize),
            additional: Vec::with_capacity(additional as usize),
            is_edns: false,
        };

        for _ in 0..queries {
            let (query, new_offset) = PartialRecord::from_offset(&bytes, offset);
            offset = new_offset;

            msg.queries.push(query);
        }

        let sections = vec![
            (answers, &mut msg.answers),
            (authorities, &mut msg.authorities),
            (additional, &mut msg.additional),
        ];
        for (count, records) in sections {
            for _ in 0..count {
                let (record, new_offset) = ResourceRecord::from_offset(&bytes, offset);
                offset = new_offset;
                if let record::RRType::OPT = record.rrtype {
                    msg.is_edns = true;
                };

                records.push(record);
            }
        }

        msg
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut header = format!("{} {:?}", self.id, self.get_opcode());
        if self.get_flag(Flags::QR) {
            header = format!("{} Response", header);

            match self.get_rcode() {
                RCode::NoError => {}
                rcode => header = format!("{} {:?}", header, rcode),
            };
        }
        let flags = vec![
            Flags::AA,
            Flags::TC,
            Flags::RD,
            Flags::RA,
            Flags::AD,
            Flags::CD,
        ];
        for flag in flags {
            if self.get_flag(flag) {
                header = format!("{} {:?}", header, flag);
            }
        }
        header = format!("+-{:-^66}-+\n| {:66} |", " Header ", header);

        let mut body = format!("+-{:-^66}-+", " Query ");
        if self.queries.len() > 0 {
            for q in self.queries.iter() {
                body = format!("{}\n| {:66} |", body, format!("{}", q));
            }
        } else {
            body = format!("{}\n|{:68}|", body, "");
        }

        let records = vec![
            ("Answer", &self.answers),
            ("Authority", &self.authorities),
            ("Additional", &self.additional),
        ];
        for (section, record) in records {
            body = format!("{}\n+-{:-^66}-+", body, format!(" {} ", section));
            if record.len() > 0 {
                for rec in record.iter() {
                    body = format!("{}\n| {:66} |", body, format!("{}", rec));
                }
            } else {
                body = format!("{}\n|{:68}|", body, "");
            }
        }

        write!(f, "{}\n{}\n+{:-^68}+", header, body, "-")
    }
}
