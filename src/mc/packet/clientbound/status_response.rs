use std::io;

use crate::mc::{packet::{InboundPacket, MCPacketHeader, MCPacket, packet_ids::STATUS_RES_PACKET_ID}};

use super::JsonResponse;



pub struct StatusResponse {
    pub json_response: JsonResponse
}

#[allow(unused)]
impl InboundPacket for StatusResponse {
    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let mut bytes = bytes.to_vec();

        let packet_header = MCPacketHeader::from_bytes(&mut bytes)?;
        if packet_header.id.value() != STATUS_RES_PACKET_ID {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid packet ID."));
        }
        
        Ok(StatusResponse{ 
            json_response: JsonResponse::from_bytes(&bytes)?
        })
    }

    fn from_data(packet: &MCPacket) -> Result<Self, io::Error> {
        if packet.header.id.value() != STATUS_RES_PACKET_ID {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid packet ID."));
        }

        Ok(StatusResponse{ 
            json_response: JsonResponse::from_bytes(&packet.data)?
        })
    }

    fn packet_id(&self) -> i32 {
        STATUS_RES_PACKET_ID
    }
}
