use crate::utils::bytes_to_u16;

#[derive(Debug)]
pub struct Message {
    id: u16,
    flags: u16,
    qry_count: u16,
    ans_count: u16,
    auth_count: u16,
    addl_count: u16,
}

impl From<&[u8]> for Message {
    fn from(bytes: &[u8]) -> Message {
        assert!(bytes.len() >= 12);

        let id = bytes_to_u16(&bytes);
        let flags = bytes_to_u16(&bytes[2..]);
        let qry_count = bytes_to_u16(&bytes[4..]);
        let ans_count = bytes_to_u16(&bytes[6..]);
        let auth_count = bytes_to_u16(&bytes[8..]);
        let addl_count = bytes_to_u16(&bytes[10..]);

        Message {
            id,
            flags,
            qry_count,
            ans_count,
            auth_count,
            addl_count,
        }
    }
}
