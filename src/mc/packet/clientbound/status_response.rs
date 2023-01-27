use std::io;

use crate::mc::{mctypes::{MCString, VarInt}, packet::{InboundPacket, read_packet_header}};

const STATUS_RES_PACKET_ID: i32 = 0x00;

pub struct StatusResponse {
    packet_size: VarInt,
    pub json_response: MCString
}

#[allow(unused)]
impl InboundPacket for StatusResponse {
    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let mut bytes = bytes.to_vec();

        let packet_header = read_packet_header(&mut bytes)?;
        if packet_header.id.value() != STATUS_RES_PACKET_ID {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid packet ID."));
        }
        
        Ok(StatusResponse{ 
            packet_size: packet_header.size,
            json_response: MCString::from_bytes(&bytes)?
        })
    }

    fn from_data(packet: &crate::mc::packet::MCPacket) -> Result<Self, io::Error> {
        if packet.header.id.value() != STATUS_RES_PACKET_ID {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid packet ID."));
        }

        Ok(StatusResponse{
            packet_size: packet.header.size.clone(),
            json_response: MCString::from_bytes(&packet.data)?
        })
    }

    fn packet_size(&self) -> VarInt {
        self.packet_size.clone()
    }

    fn packet_id(&self) -> i32 {
        STATUS_RES_PACKET_ID
    }
}
