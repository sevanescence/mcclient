use crate::mc::{mctypes::PacketBytesBuilder, packet::OutboundPacket};

#[repr(i32)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum NextState {
    STATUS = 1,
    LOGIN = 2,
}

impl Into<i32> for NextState {
    fn into(self) -> i32 {
        self as i32
    }
}

pub struct Handshake {
    pub protocol_version: i32,
    pub server_addr: String,
    pub port: u16,
    pub next_state: NextState,
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
}
