use std::{net::TcpStream, io::{self, Write, Read}, collections::VecDeque};

use mcclient::mc::{packet::{serverbound::{handshake::{Handshake, NextState}, status_request::StatusRequest}, OutboundPacket, serialize_packet}, mctypes::VarInt};

extern crate mcclient;

fn main() -> Result<(), io::Error> {
    const IP: &str = "localhost:25565";

    println!("Connecting...");
    let mut sock = TcpStream::connect(IP)?;

    let handshake = Handshake { 
        protocol_version: 760.into(), 
        server_addr: "localhost".into(),
        port: 25565_u16,
        next_state: NextState::STATUS
    };

    println!("Making handshake...");
    sock.write_all(&serialize_packet(&handshake))?;
    sock.write_all(&serialize_packet(&StatusRequest))?;
    sock.flush()?;

    println!("Reading response...");
    let mut received: Vec<u8> = vec![];
    const BUF_SIZE: usize = 256;
    let mut rx_bytes = [0_u8; BUF_SIZE];
    loop {
        let bytes_read = sock.read(&mut rx_bytes)?;

        received.extend_from_slice(&rx_bytes[..bytes_read]);
        //println!("{:?}", &rx_bytes[..bytes_read]);

        if bytes_read < BUF_SIZE {
            break;
        }
    }
    println!("Data read.");

    let mut received: VecDeque<u8> = received.into();

    let packet_size = VarInt::from_bytes(received.as_slices().0);
    for _ in 0..packet_size.len() {
        received.pop_front();
    }
    println!("{:?}", received);
    println!("{}, {}, {:?}", received.len(), packet_size.value(), packet_size.bytes());
    println!("{:?}", VarInt::from(packet_size.value()).bytes());
    // let str_size = VarInt::from_bytes(&received);
    // for _ in 0..str_size.len() {
    //     pop_remove(&mut received, 0);
    // }

    //println!("{}", String::from_utf8(received).unwrap());
    println!("{:?}", &received);
    sock.shutdown(std::net::Shutdown::Both)?;
    Ok(())
    // String::from_utf8(received)
    //     .map(|msg| println!("{}", msg))
    //     .map_err(|_| {
    //         io::Error::new(
    //             io::ErrorKind::InvalidData,
    //             "Couldn't parse received string as utf8"
    //         )
    //     })
}
