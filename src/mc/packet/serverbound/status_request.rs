use crate::mc::packet::OutboundPacket;

pub struct StatusRequest;

impl OutboundPacket for StatusRequest {
    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn packet_id(&self) -> i32 {
        0x00
    }
}
