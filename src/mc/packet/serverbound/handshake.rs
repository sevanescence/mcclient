use std::mem::size_of;

use crate::mc::{packet::OutboundPacket, mctypes::{VarInt, MCType, MCString}};

#[repr(i32)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
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

#[allow(dead_code)]
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
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

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

        size += self.protocol_version.len();
        size += self.server_addr.size();
        size += size_of::<u16>() as i32;
        size += VarInt::from(self.next_state as i32).len();

        size
    }
}
