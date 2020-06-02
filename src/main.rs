use std::net::UdpSocket;

fn bytes_to_u16(bytes: &[u8]) -> u16 {
    (bytes[0] as u16) << 8 | bytes[1] as u16
}

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
}
