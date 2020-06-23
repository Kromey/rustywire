mod record;

use bytes::{Buf, BufMut, Bytes};
pub use record::{Class, Flags, OpCode, RCode, RRType};
use record::{PartialRecord, ResourceRecord};
use std::{fmt, str};

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

        bytes.put_u16(self.id);
        bytes.put_u16(self.flags);

        bytes.put_u16(self.queries.len() as u16);
        bytes.put_u16(self.answers.len() as u16);
        bytes.put_u16(self.authorities.len() as u16);
        bytes.put_u16(self.additional.len() as u16);

        for q in self.queries.iter() {
            bytes.extend(Message::name_as_bytes(&q.label));
            bytes.put_u16(q.rrtype as u16);
            bytes.put_u16(q.class as u16);
        }
        for records in vec![&self.answers, &self.authorities, &self.additional] {
            for record in records.iter() {
                bytes.extend(Message::name_as_bytes(&record.label));
                bytes.put_u16(record.rrtype as u16);
                bytes.put_u16(record.class as u16);
                bytes.put_u32(record.ttl);
                bytes.put_u16(record.data.len() as u16);
                bytes.extend(&record.data);
            }
        }

        bytes
    }

    fn name_as_bytes(name: &str) -> Vec<u8> {
        if name == "." {
            return vec![0];
        }

        let mut bytes = Vec::new();
        for label in name.split('.') {
            bytes.put_u8(label.len() as u8);
            bytes.extend(label.as_bytes());
        }

        bytes
    }

    fn get_name(bytes: &mut Bytes) -> String {
        let mut name = String::new();
        loop {
            let len = bytes.get_u8() as usize;

            if len >= 0xC0 {
                bytes.advance(1);
                name.push('.');
                break;
            }

            if len == 0 {
                break;
            }

            name.push_str(str::from_utf8(&bytes.slice(..len)).unwrap());
            name.push('.');
            bytes.advance(len);
        }

        if name.len() > 0 {
            name
        } else {
            ".".into()
        }
    }
}

impl From<Bytes> for Message {
    fn from(mut bytes: Bytes) -> Message {
        assert!(bytes.len() >= 12);

        let id = bytes.get_u16();
        let flags = bytes.get_u16();

        println!("{} {:04X}", id, flags);

        let queries = bytes.get_u16();
        let answers = bytes.get_u16();
        let authorities = bytes.get_u16();
        let additional = bytes.get_u16();

        println!("{} {} {} {}", queries, answers, authorities, additional);

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
            let name = Message::get_name(&mut bytes);
            let qtype = bytes.get_u16();
            let class = bytes.get_u16();

            let query = PartialRecord {
                label: name,
                rrtype: qtype.into(),
                class: class.into(),
            };

            println!("{}", query);

            msg.queries.push(query);
        }

        let sections = vec![
            (answers, &mut msg.answers),
            (authorities, &mut msg.authorities),
            (additional, &mut msg.additional),
        ];
        for (count, records) in sections {
            for _ in 0..count {
                let name = Message::get_name(&mut bytes);
                let rtype = bytes.get_u16();
                let class = bytes.get_u16();
                let ttl = bytes.get_u32();
                let rdlen = bytes.get_u16() as usize;
                let rdata = bytes.slice(..rdlen);
                bytes.advance(rdlen);

                let rtype = rtype.into();

                let record = ResourceRecord {
                    label: name,
                    rrtype: rtype,
                    class: match rtype {
                        RRType::OPT => Class::NONE,
                        _ => Class::from(class),
                    },
                    ttl,
                    data: rdata.into(),
                };

                println!("{}", record);

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
