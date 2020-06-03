use crate::utils::bytes_to_u16;

#[derive(Debug)]
pub struct Header {
    id: u16,
    flags: u16,
    qry_count: u16,
    ans_count: u16,
    auth_count: u16,
    addl_count: u16,
}

impl From<&&mut [u8]> for Header {
    fn from(bytes: &&mut [u8]) -> Header {
        assert!(bytes.len() >= 12);

        let id = bytes_to_u16(&bytes);
        let flags = bytes_to_u16(&bytes[2..]);
        let qry_count = bytes_to_u16(&bytes[4..]);
        let ans_count = bytes_to_u16(&bytes[6..]);
        let auth_count = bytes_to_u16(&bytes[8..]);
        let addl_count = bytes_to_u16(&bytes[10..]);

        Header {
            id,
            flags,
            qry_count,
            ans_count,
            auth_count,
            addl_count,
        }
    }
}
