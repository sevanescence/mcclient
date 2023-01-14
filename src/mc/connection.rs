use std::net::TcpStream;

const PROTOCOL_VERSION: i32 = 760;

/// Represents a connection stream to an offline Minecraft server.
/// <br>
/// The handshake packet is sent when either a status or login request
/// is made. The stream itself attempts to open upon construction of
/// the object.
pub struct OfflineConnection {
    stream: TcpStream,
    username: String,
    domain: String,
    port: u16
}

impl OfflineConnection {
    pub fn new(domain: String, port: u16, username: String) -> Self {
        let ip = format!("{}:{}", domain, port);

        let stream = match TcpStream::connect(ip) {
            Ok(stream) => stream,
            Err(msg) => { panic!("Could not connect: {}", msg); }
        };

        OfflineConnection { stream, username, domain, port }
    }

    // TODO: Make this capturable via pattern matching by returning a Result.
    pub fn status(&mut self) -> () {
        // https://wiki.vg/Protocol#Status_Response 
        const MAX_BUFSIZE: usize = 2800;
        let mut buf: [u8; MAX_BUFSIZE] = [0; MAX_BUFSIZE];

        // let handshake_packet = mc_protocol::serialize_packet(&Handshake::new(
        //     PROTOCOL_VERSION,
        //     self.domain.to_string(),
        //     self.port,
        //     NextState::STATUS
        // ));


        ()
    }

    // TODO: Make this capturable via pattern matching by returning a Result.
    pub fn login(&mut self) {
        const LOGIN_START_PACKET_ID: i32 = 0x00;

        

        // let packet_id_varint = mc_protocol::to_varint(LOGIN_START_PACKET_ID);

        // let packet: Vec<u8> = {
        //     let mut packet = Vec::<u8>::new();
        //     let mut data = Vec::<u8>::new();

        //     // No UUID
        //     data.append(&mut mc_protocol::to_varint((self.username.len() + 1) as i32));
        //     data.append(&mut self.username.as_bytes().to_vec());
        //     data.push(0u8);

        //     packet.append(&mut mc_protocol::to_varint((data.len() + packet_id_varint.len()) as i32));
        //     packet.append(&mut packet_id_varint.clone());
        //     packet.append(&mut data);

        //     packet
        // };

        // println!("{:?}", packet);
    }
}