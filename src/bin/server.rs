use std::{io::Read, net};

use tcp_udp_toe::TicField;

fn handle_stream(mut stream: net::TcpStream) {
    println!("Got a connection");
    let mut buf = [0, 0, 0, 0];
    let size = stream.read(&mut buf).expect("Couldn't read from stream");
    println!("We got {} bytes: '{}'", size, TicField::from_bytes(&buf));
}

fn main() {
    let listener = net::TcpListener::bind("localhost:2024").expect("Port is not available");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream),
            Err(e) => {
                println!("e: {}", e)
            }
        }
    }
}
