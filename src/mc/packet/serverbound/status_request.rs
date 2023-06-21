use crate::mc::packet::{OutboundPacket, packet_ids};

pub struct StatusRequest;

impl OutboundPacket for StatusRequest {
    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn packet_id(&self) -> i32 {
        packet_ids::serverbound::STATUS_REQUEST
    }
}
