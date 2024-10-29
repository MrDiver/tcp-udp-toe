use std::{io::Write, net};

fn main() {
    let mut stream =
        net::TcpStream::connect("localhost:2024").expect("Couldn't connect to this address :(");
    stream.write("Na du".as_bytes()).expect("Could not send");
}
