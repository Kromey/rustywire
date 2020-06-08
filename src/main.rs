use rustywire::message::{Message, RCode};
use std::net::UdpSocket;

fn main() {
    println!("Hello, world!");
    println!("Starting UDP socket...");

    let socket = UdpSocket::bind("127.0.0.1:3553").expect("Couldn't bind to address");
    let mut buf = [0u8; 512];

    println!("Bound! Listening...");

    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("No data!");
    println!("Got {} bytes from {:?}", number_of_bytes, src_addr);

    let data = &buf[..number_of_bytes];
    let line_size = 12;
    for offset in (0..number_of_bytes).step_by(line_size) {
        print!("{:04} ", offset);
        for byte in data[offset..(offset + line_size).min(number_of_bytes)].iter() {
            print!("{:02X} ", byte);
        }
        println!("");
    }
    println!("");

    let message = Message::from(buf[..number_of_bytes].to_vec());
    println!("{}\n", message);

    let mut resp = message.into_response();
    resp.set_rcode(RCode::ServFail);
    println!("{}\n", resp);

    let bytes = resp.as_bytes();
    let line_size = 12;
    for offset in (0..bytes.len()).step_by(line_size) {
        print!("{:04} ", offset);
        for byte in bytes[offset..(offset + line_size).min(bytes.len())].iter() {
            print!("{:02X} ", byte);
        }
        println!("");
    }
    println!("");

    socket
        .send_to(&bytes, &src_addr)
        .expect("Failed to send response");
}
