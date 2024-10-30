use std::{io::Write, net};
use tcp_udp_toe::{Player, TicField};

fn main() {
    let mut tmp = TicField::new();
    tmp.set_field(0, Player::O).expect("Field already set");
    let mut stream =
        net::TcpStream::connect("localhost:2024").expect("Couldn't connect to this address :(");
    println!("Sending {}", tmp);
    stream.write(&tmp.as_bytes()).expect("Could not send");
}
