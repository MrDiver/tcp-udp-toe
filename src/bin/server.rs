use std::{io::Read, net};

fn handle_stream(mut stream: net::TcpStream) {
    println!("Got a connection");
    let mut buf = String::new();
    let size = stream
        .read_to_string(&mut buf)
        .expect("Couldn't read from stream");
    println!("We got {} bytes: '{}'", size, buf);
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
