use std::mem::size_of;

use crate::mc::{packet::OutboundPacket, mctypes::{VarInt, MCType, MCString, PacketBytesBuilder}};

#[repr(i32)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum NextState {
    STATUS = 1,
    LOGIN = 2
}

impl Into<i32> for NextState {
    fn into(self) -> i32 {
        self as i32
    }
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
        PacketBytesBuilder::new()
            .append_i32_as_varint(self.protocol_version)
            .append_string(self.server_addr.clone())
            .append_u16(self.port)
            .append_i32_as_varint(self.next_state.into())
        .byte_buffer
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
