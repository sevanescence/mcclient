use std::mem::size_of;

use crate::mc::{packet::OutboundPacket, mctypes::{VarInt, MCType, MCString, PacketBytesBuilder}};

#[repr(i32)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum NextState {
    STATUS = 1,
    LOGIN = 2
}

// pub struct Handshake {
//     pub protocol_version: VarInt,
//     pub server_addr: MCString,
//     pub port: u16,
//     pub next_state: NextState
// }

pub struct Handshake {
    pub protocol_version: i32,
    pub server_addr: String,
    pub port: u16,
    pub next_state: NextState
}

impl OutboundPacket for Handshake {
    fn to_bytes(&self) -> Vec<u8> {
        // let mut bytes = Vec::<u8>::new();

        // bytes.append(&mut VarInt::from(self.protocol_version).to_bytes());
        // bytes.append(&mut VarInt::from(self.protocol_version).to_bytes());
        // bytes.append(&mut self.port.to_be_bytes().to_vec());
        // bytes.append(&mut VarInt::from_i32(self.next_state as i32).to_bytes());

        // bytes

        let mut builder = PacketBytesBuilder::new();

        builder.append_i32_as_varint(self.protocol_version);
        builder.append_string(self.server_addr.clone());
        builder.append_u16(self.port);
        builder.append_i32_as_varint(self.next_state as i32);

        builder.byte_buffer
    }

    fn packet_id(&self) -> i32 {
        0x00
    }

    fn len(&self) -> i32 {
        let mut size: i32 = 0;

        //size += self.protocol_version.len();
        //size += self.server_addr.size();
        size += VarInt::from_i32(self.protocol_version).len();
        size += MCString::from(self.server_addr.clone()).size();
        size += size_of::<u16>() as i32;
        size += VarInt::from(self.next_state as i32).len();

        size
    }
}
