use std::io;

use crate::mc::{
    mctypes::JsonResponse,
    packet::{ClientboundRawPacket, InboundPacket, packet_ids},
};

pub struct StatusResponse {
    pub json_response: JsonResponse,
}

#[allow(unused)]
impl InboundPacket for StatusResponse {
    fn from_data(packet: &ClientboundRawPacket) -> Result<Self, io::Error> {
        let res = JsonResponse::from_bytes(&packet.data).expect("failed here");
        Ok(StatusResponse {
            json_response: res,
        })
    }

    fn packet_id(&self) -> i32 {
        packet_ids::clientbound::STATUS_RESPONSE
    }
}
