use std::{
    net::TcpStream, mem::size_of, io::{Write, Read}, str::from_utf8,
};

use crate::mc_protocol::VarInt;

const PROTOCOL_VERSION: i32 = 760;
#[repr(i32)]
#[derive(Clone, Copy)]
enum NextState {
    STATUS = 1,
    //LOGIN = 2
}

pub mod mc_protocol {
    use crate::OutboundPacket;

    /// Serialize a serverbound packet to be sent to a server.
    pub fn serialize_packet(data: Box<dyn OutboundPacket>) -> Vec<u8> {
        let mut bytes  = Vec::<u8>::new();

        let mut packet_id = to_varint(data.packet_id());
        let mut len = to_varint(packet_id.len() as i32 + data.len());

        bytes.append(&mut len);
        bytes.append(&mut packet_id);
        bytes.append(&mut data.to_vec());

        bytes
    }

    /// A `VarInt` is a variable-length data type encoding a two's
    /// complement signed 32-bit integer. A `VarInt` can be anywhere
    /// between 1 and 5 bytes. https://wiki.vg/Protocol#VarInt_and_VarLong
    /// <br>
    /// This structure is meant purely for data I/O and should not be used
    /// to perform any sort of arithmetic.
    #[derive(Clone)]
    pub struct VarInt {
        bytes: Vec<u8>,
        value: i32
    }

    impl From<i32> for VarInt {
        /// Creates a `VarInt` representation of `value`.
        fn from(value: i32) -> Self {
            VarInt{ bytes: to_varint(value), value }
        }
    }

    impl From<&[u8]> for VarInt {
        /// Creates a `VarInt` from a slice `&[u8]` whose leading bytes represent
        /// a `VarInt` string between 1 and 5 bytes. Because it is expected that the
        /// data being passed to `from_bytes` is a slice of a Minecraft packet, and
        /// the size of a `VarInt` is variadic, a slice larger than the encompassed
        /// number can be passed without unexpected error.
        /// 
        /// # Panics
        /// 
        /// The parsing of the leading bytes to a `VarInt` will panic if the number
        /// is evaluated to greater than 5 bytes in size. This can be caused by
        /// either the wrong data type being read or the bytes being badly formatted.
        fn from(bytes: &[u8]) -> Self {
            VarInt{ bytes: bytes.to_vec(), value: from_varint_bytes(bytes) }
        }
    }

    impl VarInt {
        /// Retrieves the byte size of the `VarInt`.
        pub fn len(&self) -> i32 {
            self.bytes.len() as i32
        }

        /// Returns a slice of this `VarInt`'s byte array representation.
        pub fn as_bytes(&self) -> &[u8] {
            &self.bytes
        }

        /// Returns the numerical equivalent of this `VarInt`.
        pub fn value(&self) -> i32 {
            self.value
        }

        /// Sets the value of this `VarInt` to represent the `value` passed. This function 
        /// may be used in place of `VarInt::from_i32()` when reinitializing a `VarInt` 
        /// is not favorable.
        pub fn set(&mut self, value: i32) {
            self.value = value;
            self.bytes = to_varint(self.value);
        }
    }

    fn from_varint_bytes(bytes: &[u8]) -> i32 {
        let mut value = 0;
        let mut pos = 0;

        const SEGMENT_BITS: i32 = 0x7F;
        const CONTINUE_BIT: i32 = 0x80;

        for b in bytes.iter() {
            value |= ((*b as i32) & SEGMENT_BITS) << pos;

            if (*b as i32) & CONTINUE_BIT == 0 {
                break;
            }

            pos += 7;

            if pos >= 32 {
                panic!("VarInt is too big (>5 bytes)");
            }
        }

        value
    }

    /// Reads a varint from the front of the buffer and returns
    /// a tuple of the value, and a slice of the buffer after the read bytes.
    pub fn read_varint(mut bytes: &[u8]) -> (i32, &[u8]) {
        let mut value = 0;
        let mut pos = 0;

        const SEGMENT_BITS: i32 = 0x7F;
        const CONTINUE_BIT: i32 = 0x80;

        for b in bytes.iter() {
            value |= ((*b as i32) & SEGMENT_BITS) << pos;
            bytes = &bytes[1..];

            if (*b as i32) & CONTINUE_BIT == 0 {
                break;
            }

            pos += 7;

            if pos >= 32 {
                panic!("VarInt is too big (>5 bytes)");
            }
        }

        (value, bytes)
    }

    pub fn to_varint(mut value: i32) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
    
        const SEGMENT_BITS: i32 = 0x7F;
        const CONTINUE_BIT: i32 = 0x80;
    
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                bytes.push(value as u8);
                break;
            }
    
            bytes.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);
    
            // https://stackoverflow.com/a/70212287
            value = ((value as u32) >> 7) as i32;
        }
    
        bytes
    }
}

/// Interfaces serverbound packets. Structs implementing this trait are
/// expected to be mcproto-compliant packets; transfering malformatted
/// packets will result in undefined behavior.
pub trait OutboundPacket {
    /// Serializes the internal packet data into an array of bytes.
    fn to_vec(&self) -> Vec<u8>;
    /// Retrieves the ID of this packet. This can be a compile-time constant.
    /// Will eventually refactor.
    fn packet_id(&self) -> i32;
    /// Get length of packet (excluding length of Packet ID)
    fn len(&self) -> i32;
}

struct TestStruct {

}

impl InboundPacket for TestStruct {
    fn packet_id(&self) -> i32 {
        0x00
    }
}

pub trait InboundPacket {
    fn packet_id(&self) -> i32;
    
}

struct StatusResponse {
    json: String,
}

// Unimplemented
// struct PingResponse {

// }

// struct ClientboundStatusResponse {

// }

struct Handshake {
    pub protocol_version: i32,
    pub server_addr: String,
    pub port: u16,
    pub next_state: NextState
}

impl Handshake {
    pub fn new(protocol_version: i32, server_addr: String, port: u16, next_state: NextState) -> Self {
        Handshake { protocol_version, server_addr, port, next_state }
    }
}

impl OutboundPacket for Handshake {
    fn to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        bytes.append(&mut mc_protocol::to_varint(self.protocol_version));
        // TODO write handler functions for serializing specific data types.
        let mut server_addr_bytes = self.server_addr.as_bytes().to_vec();
        bytes.append(&mut mc_protocol::to_varint(server_addr_bytes.len() as i32));
        bytes.append(&mut server_addr_bytes);
        bytes.append(&mut self.port.to_be_bytes().to_vec());
        bytes.append(&mut mc_protocol::to_varint(self.next_state as i32));

        bytes
    }

    fn packet_id(&self) -> i32 {
        0x00
    }

    fn len(&self) -> i32 {
        let mut size: i32 = 0;

        // since i dont really do any computations on varints i might just
        // make the member types of the numbers varints instead of integers
        size += mc_protocol::to_varint(self.protocol_version).len() as i32;
        size += mc_protocol::to_varint(self.server_addr.as_bytes().len() as i32).len() as i32;
        size += self.server_addr.as_bytes().len() as i32;
        size += size_of::<u16>() as i32; // size of server port
        size += mc_protocol::to_varint(self.next_state as i32).len() as i32;

        size
    }
}

/// Represents a connection stream to an offline Minecraft server.
/// <br>
/// The handshake packet is sent when either a status or login request
/// is made. The stream itself attempts to open upon construction of
/// the object.
struct OfflineConnection {
    stream: TcpStream,
    // Will you STOP telling me username is useless Rust? I KNOW WHAT IM FUCKING DOING!!!
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
    pub fn status(&mut self) -> StatusResponse {
        // https://wiki.vg/Protocol#Status_Response 
        const MAX_BUFSIZE: usize = 2800;
        let mut buf: [u8; MAX_BUFSIZE] = [0; MAX_BUFSIZE];

        let handshake_packet = mc_protocol::serialize_packet(Box::new(Handshake::new(
            PROTOCOL_VERSION,
            self.domain.to_string(),
            self.port,
            NextState::STATUS
        )));

        match self.stream.write(&handshake_packet) {
            Ok(_) => (),
            Err(msg) => panic!("Could not send handshake: {}", msg)
        }

        // https://wiki.vg/Protocol#Status_Request
        let status_request_packet: [u8; 2] = [0x01, 0x00];
        match self.stream.write(&status_request_packet) {
            Ok(_) => (),
            Err(msg) => panic!("Could not send status request: {}", msg)
        }

        // TODO get response packet

        let read = match self.stream.read(&mut buf) {
            Ok(size) => size,
            Err(e) => panic!("Could not read: {}", e)
        };
        //println!("{}", read);

        let (packet_len, b) = mc_protocol::read_varint(&buf);
        let (packet_id, b) = mc_protocol::read_varint(&b);
        let header_len 
            = mc_protocol::to_varint(packet_len).len() + mc_protocol::to_varint(packet_id).len();

        match packet_id {
            0 => (),
            _ => panic!("Invalid packet id read: {}", packet_id)
        }

        // Read string data from packet
        // TODO wrap this in a handler, this is just hardcode.
        let (data_strlen, b) = mc_protocol::read_varint(&b);
        let data_header_len = mc_protocol::to_varint(data_strlen).len() as usize;

        let mut data: String = match from_utf8(&b[..read - (header_len + data_header_len) as usize]) {
            Ok(s) => s.to_owned(),
            Err(e) => panic!("Could not parse bytes: {}", e)
        };

        let mut total_read = read;
        while total_read >= (packet_len as usize) {
            let read = match self.stream.read(&mut buf) {
                Ok(r) => r,
                Err(e) => panic!("Could not parse initially: {}", e)
            };

            data.push_str(match from_utf8(&buf[..read]) {
                Ok(s) => s,
                Err(e) => panic!("Could not parse: {}", e)
            });

            total_read += read;
        }

        StatusResponse { json: data }
    }

    // TODO: Make this capturable via pattern matching by returning a Result.
    pub fn login(&mut self) {
        // NOTE just hardcode for now, to get the ball rolling.
        const LOGIN_START_PACKET_ID: i32 = 0x00;
        let packet_id_varint = mc_protocol::to_varint(LOGIN_START_PACKET_ID);

        let packet: Vec<u8> = {
            let mut packet = Vec::<u8>::new();
            let mut data = Vec::<u8>::new();

            // No UUID
            data.append(&mut mc_protocol::to_varint((self.username.len() + 1) as i32));
            data.append(&mut self.username.as_bytes().to_vec());
            data.push(0u8);

            packet.append(&mut mc_protocol::to_varint((data.len() + packet_id_varint.len()) as i32));
            packet.append(&mut packet_id_varint.clone());
            packet.append(&mut data);

            packet
        };

        println!("{:?}", packet);
    }
}

// mod serverbound_request_status {
//     pub fn get_bytes() -> Vec<u8> {
//         [0x01u8, 0x00u8].to_vec()
//     }
// }

fn main() {
    // Serverbound packet structure
    // <length: VarInt> <packet ID: VarInt> <data>
    // Handshake packet structure
    // <protover: VarInt> <addr: String(255)> <port: ushort> <next_state: VarInt Enum>

    let i = 375;
    let vi = VarInt::from(i);
    let n = VarInt::from(vi.as_bytes());
    println!("{:?}", n.value());

    // Hardcoded constants for now
    const DOMAIN: &str = "localhost";
    const PORT: u16 = 25565;
    const USERNAME: &str = "MakotoII";

    println!("Connecting to server {}:{}", DOMAIN, PORT);

    let mut connection = OfflineConnection::new(
        DOMAIN.to_string(), PORT, USERNAME.to_string()
    );

    println!("Connection established. Requesting status...");
    let status = connection.status();

    println!("Status: {}", status.json);

    println!("Logging in...");
    connection.login();
}
