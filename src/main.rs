use rustywire::message::Header;
use rustywire::utils::{bytes_to_u16, bytes_to_u32};
use std::net::UdpSocket;
use std::str;

fn main() {
    println!("Hello, world!");
    println!("Starting UDP socket...");

    let socket = UdpSocket::bind("127.0.0.1:3553").expect("Couldn't bind to address");
    let mut buf = [0u8; 512];

    println!("Bound! Listening...");

    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("No data!");
    println!("Got {} bytes from {:?}", number_of_bytes, src_addr);

    let data = &mut buf[..number_of_bytes];
    for byte in data.iter() {
        print!("{:02X} ", byte);
    }
    println!("");

    let id = bytes_to_u16(&data[0..=1]); //Query ID
    let qr = data[2] >> 7; //0 = query; 1 = response
    let opcode = (data[2] >> 3) & 15; //OPCODE; see RFC1035
    let aa = (data[2] >> 2) & 1; //Authoritative Answer [response]
    let tc = (data[2] >> 1) & 1; //Truncated
    let rd = data[2] & 1; //Recursion Desired [query]

    println!("Flags: {:08b} {:08b}", data[2], data[3]);
    println!("ID: {} (0x{:04X})", id, id);
    println!("QR: {} (0x{:04X})", qr, qr);
    println!("OP: {} (0x{:04X})", opcode, opcode);
    println!("AA: {} (0x{:04X})", aa, aa);
    println!("TC: {} (0x{:04X})", tc, tc);
    println!("RD: {} (0x{:04X})", rd, rd);
    println!("");

    let ra = data[3] >> 7; //Recursion Available [response]
    let z = (data[3] >> 6) & 1; //Zero
    let ad = (data[3] >> 5) & 1; //Authenticated Data [DNSSEC; query="I want", response="I did"]
    let cd = (data[3] >> 4) & 1; //Checking Disabled [query; disable DNSSEC validation]
    let rcode = data[3] & 15; //Response CODE; see RFC1035

    println!("RA: {} (0x{:04X})", ra, ra);
    println!(" Z: {} (0x{:04X})", z, z);
    println!("AD: {} (0x{:04X})", ad, ad);
    println!("CD: {} (0x{:04X})", cd, cd);
    println!("RCODE: (0x{:04X})", rcode);
    println!("");

    let qdcount = bytes_to_u16(&data[4..=5]); // query count
    let ancount = bytes_to_u16(&data[6..=7]); // answer count
    let nscount = bytes_to_u16(&data[8..=9]); // name server count
    let arcount = bytes_to_u16(&data[10..=11]); // additional record count

    println!("QDCOUNT: {}", qdcount);
    println!("ANCOUNT: {}", ancount);
    println!("NSCOUNT: {}", nscount);
    println!("ARCOUNT: {}", arcount);

    let header = Header::from(&data);
    println!("{:#?}", header);

    let mut queries: Vec<Vec<&str>> = Vec::new();
    let records = &data[12..];
    let mut offset = 0;

    for _ in 0..qdcount {
        let mut query = Vec::new();

        while records[offset] > 0 {
            let length = records[offset] as usize;
            offset += 1;
            query.push(str::from_utf8(&records[offset..(offset+length)]).unwrap());

            offset += length;
        }
        // Move past the last length field we just read
        offset += 1;

        // Read the QTYPE field
        //let qtype = bytes_to_u16(&records[offset..=offset+1]);
        query.push(str::from_utf8(&records[offset..=offset+1]).unwrap());
        offset += 2;
        // Read the QCLASS field
        query.push(str::from_utf8(&records[offset..=offset+1]).unwrap());
        offset += 2;

        queries.push(query);
    }
    println!("\nQueries:\n{:#?}", queries);

    println!("\nAdditional Records:");
    for _ in 0..arcount {
        let mut record = Vec::new();

        // Read the name, same as before
        while records[offset] > 0 {
            let length = records[offset] as usize;
            offset += 1;
            record.push(str::from_utf8(&records[offset..(offset+length)]).unwrap());

            offset += length;
        }
        // Move past the last length field we just read
        offset += 1;
        println!("\n{:?}", record);

        println!("TYPE: {}", bytes_to_u16(&records[offset..=offset+1]));
        offset += 2;

        println!("CLASS: {}", bytes_to_u16(&records[offset..=offset+1]));
        offset += 2;

        // TTL is 32-bit, unlike other 16-bite values
        println!("TTL: {}", bytes_to_u32(&records[offset..=offset+3]));
        offset += 4;

        let rdlength = bytes_to_u16(&records[offset..=offset+1]) as usize;
        println!("RDLENGTH: {}", rdlength);
        offset += 2;

        print!("RDATA:");
        let rdata = &records[offset..offset+rdlength];
        for byte in rdata.iter() {
            print!(" {:02X}", byte);
        }
        println!("");
        offset += rdlength;
    }
}
