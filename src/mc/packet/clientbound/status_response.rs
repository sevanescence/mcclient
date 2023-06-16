use std::io;

use crate::mc::{packet::{InboundPacket, MCPacket, packet_ids::STATUS_RES_PACKET_ID}, mctypes::JsonResponse};

pub struct StatusResponse {
    pub json_response: JsonResponse
}

#[allow(unused)]
impl InboundPacket for StatusResponse {
    fn from_data(packet: &MCPacket) -> Result<Self, io::Error> {
        Ok(StatusResponse{ 
            json_response: JsonResponse::from_bytes(&packet.data)?
        })
    }

    fn packet_id(&self) -> i32 {
        STATUS_RES_PACKET_ID
    }
}
