use std::mem::size_of;

use crate::mc::{packet::OutboundPacket, mctypes::{to_varint, VarInt, MCType, MCString}};

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum NextState {
    STATUS = 1,
    LOGIN = 2
}

pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_addr: MCString,
    pub port: u16,
    pub next_state: NextState
}

impl Handshake {
    pub fn new(protocol_version: i32, server_addr: String, port: u16, next_state: NextState) -> Self {
        Handshake { 
            protocol_version: VarInt::from_i32(protocol_version), 
            server_addr: MCString::from(server_addr), 
            port, 
            next_state
        }
    }
}

impl OutboundPacket for Handshake {
    fn to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        // bytes.append(&mut mc_protocol::to_varint(self.protocol_version));
        // // TODO write handler functions for serializing specific data types.
        // let mut server_addr_bytes = self.server_addr.as_bytes().to_vec();
        // bytes.append(&mut mc_protocol::to_varint(server_addr_bytes.len() as i32));
        // bytes.append(&mut server_addr_bytes);
        // bytes.append(&mut self.port.to_be_bytes().to_vec());
        // bytes.append(&mut mc_protocol::to_varint(self.next_state as i32));

        bytes.append(&mut self.protocol_version.to_bytes());
        bytes.append(&mut self.server_addr.to_bytes());
        bytes.append(&mut self.port.to_be_bytes().to_vec());
        bytes.append(&mut VarInt::from_i32(self.next_state as i32).to_bytes());

        bytes
    }

    fn packet_id(&self) -> i32 {
        0x00
    }

    fn len(&self) -> i32 {
        let mut size: i32 = 0;

        // since i dont really do any computations on varints i might just
        // make the member types of the numbers varints instead of integers

        // size += mc_protocol::to_varint(self.protocol_version).len() as i32;
        // size += mc_protocol::to_varint(self.server_addr.as_bytes().len() as i32).len() as i32;
        // size += self.server_addr.as_bytes().len() as i32;
        // size += size_of::<u16>() as i32; // size of server port
        // size += mc_protocol::to_varint(self.next_state as i32).len() as i32;

        size += self.protocol_version.len();
        size += self.server_addr.size();
        size += size_of::<u16>() as i32;
        size += VarInt::from(self.next_state as i32).len();

        size
    }
}
