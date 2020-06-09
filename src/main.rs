use rustywire::message::{Message, RCode};
use rustywire::utils::dump_hex;
use std::net::{Ipv4Addr, UdpSocket};

fn main() {
    println!("Hello, world!");
    println!("Starting UDP socket...");

    let socket = UdpSocket::bind("127.0.0.1:3553").expect("Couldn't bind to address");
    let mut buf = [0u8; 512];

    println!("Bound! Listening...");

    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("No data!");
    println!("Got {} bytes from {:?}", number_of_bytes, src_addr);

    let data = &buf[..number_of_bytes];
    dump_hex(&data);

    let message = Message::from(buf[..number_of_bytes].to_vec());
    println!("RECEIVED:\n{}\n", message);

    let mut upstream = message.clone();
    {
        let addr = Ipv4Addr::UNSPECIFIED;
        let port = 34567;
        let sock = UdpSocket::bind((addr, port)).expect("Could not bind to port");
        sock.connect("1.1.1.1:53").expect("Could not connect");
        sock.send(&upstream.as_bytes()).expect("Could not send");

        let mut buf = [0u8; 512];
        let received = sock.recv(&mut buf).expect("Could not receive reply");

        dump_hex(&buf[..received]);
        upstream = Message::from(buf[..received].to_vec());
        println!("UPSTREAM REPLY:\n{}\n", upstream);
    }

    let mut resp = message.into_response();
    resp.set_rcode(RCode::ServFail);
    println!("SENDING:\n{}\n", resp);

    let bytes = resp.as_bytes();
    dump_hex(&bytes);

    socket
        .send_to(&bytes, &src_addr)
        .expect("Failed to send response");
}
