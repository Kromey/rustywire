use bytes::BytesMut;
use rustywire::message::Message;
use rustywire::utils::dump_hex;
use std::net::{Ipv4Addr, UdpSocket};

fn main() {
    println!("Hello, world!");
    println!("Starting UDP socket...");

    let socket = UdpSocket::bind("127.0.0.1:3553").expect("Couldn't bind to address");
    let mut buf = BytesMut::with_capacity(512);

    /// Safety: We've just allocated these bytes, so despite them being uninitialized
    /// this is okay because they're "ours" anyway. For some reason we have to do this
    /// in order to use this as our socket buffer.
    unsafe {
        buf.set_len(512);
    }

    println!("Bound! Listening...");

    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("No data!");
    println!("Got {} bytes from {:?}", number_of_bytes, src_addr);

    buf.truncate(number_of_bytes);
    let data = buf.freeze();
    dump_hex(&data);

    let message = Message::from(data);
    println!("RECEIVED:\n{}\n", message);

    let mut upstream = message.clone();
    {
        let addr = Ipv4Addr::UNSPECIFIED;
        let port = 34567;
        let sock = UdpSocket::bind((addr, port)).expect("Could not bind to port");
        sock.connect(include_str!(".upstream").trim_end())
            .expect("Could not connect");
        sock.send(&upstream.as_bytes()).expect("Could not send");

        let mut buf = BytesMut::with_capacity(512);

        /// Safety: See above where we're doing the same thing
        unsafe {
            buf.set_len(512);
        }

        let received = sock.recv(&mut buf).expect("Could not receive reply");

        buf.truncate(received);
        let data = buf.freeze();

        dump_hex(&data);
        upstream = Message::from(data);
        println!("UPSTREAM REPLY:\n{}\n", upstream);
    }

    println!("SENDING:\n{}\n", upstream);

    let bytes = upstream.as_bytes();
    dump_hex(&bytes);

    socket
        .send_to(&bytes, &src_addr)
        .expect("Failed to send response");
}
