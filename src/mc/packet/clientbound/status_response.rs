use crate::mc::{mctypes::MCString, packet::InboundPacket};

pub struct StatusResponse {
    pub json_response: MCString
}

#[allow(unused)]
impl InboundPacket for StatusResponse {
    fn from_bytes(bytes: &[u8]) -> Result<Box<Self>, std::io::Error> {
        Ok(Box::new(StatusResponse{ json_response: "unimplemented".into() }))
    }

    fn packet_id(&self) -> i32 {
        0x00
    }
}
