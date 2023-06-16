use std::io;

use super::mctypes::{VarInt, MCType};

pub mod serverbound;
pub mod clientbound;
pub mod packet_ids;

/// Interfaces serverbound packets. Structs implementing this trait are
/// expected to be mcproto-compliant packets; transfering malformatted
/// packets will result in undefined behavior.
pub trait OutboundPacket {
    /// Serializes the internal packet data into an array of bytes.
    fn to_bytes(&self) -> Vec<u8>;
    /// Retrieves the ID of this packet. This can be a compile-time constant.
    /// Will eventually refactor.
    fn packet_id(&self) -> i32;
    /// Get length of packet (excluding length of Packet ID)
    fn len(&self) -> i32;
}

/// Interfaced clientbound packets. Struct implementing this trait
/// are expected to be mcproto-compliant packets which are parsed
/// from an array of bytes retrieved from a server.
pub trait InboundPacket: Sized {
    /// Attempts to deserialize the given packet into the implied packet type.
    /// # Returns
    /// The self-referencial packet.
    /// # Errors
    /// This function may return an error when the provided packet data
    /// is ill-formed or the internal types are not properly parsed.
    fn from_data(packet: &MCPacket) -> Result<Self, io::Error>;

    /// Retrieves the ID of this inbound packet.
    fn packet_id(&self) -> i32;
}

/// Serialize a serverbound packet to be sent to a server.
/// Packet structure: https://wiki.vg/Protocol#Packet_format
pub fn serialize_packet(data: &dyn OutboundPacket) -> Vec<u8> {
    let mut serialized_packet_bytes = Vec::<u8>::new();

    let packet_id = VarInt::from_i32(data.packet_id());
    let size = data.len() + packet_id.size();
    serialized_packet_bytes.append(&mut VarInt::from(size).to_bytes());
    serialized_packet_bytes.append(&mut packet_id.to_bytes());
    serialized_packet_bytes.append(&mut data.to_bytes());

    serialized_packet_bytes
}

pub struct MCPacketHeader{ pub size: VarInt, pub id: VarInt }

/// Attempts to parse a packet header from bytes, consuming the `VarInt` elements of
/// the `Vec<u8>` passed.
/// # Errors
/// This function may return an `InvalidData` error if the `VarInt` bytes cannot be
/// properly parsed.
fn read_packet_header(bytes: &mut Vec<u8>) -> Result<MCPacketHeader, io::Error> {
    let packet_size = VarInt::from_vec_front(bytes)?;
    let packet_id = VarInt::from_vec_front(bytes)?;

    Ok(MCPacketHeader{ size: packet_size, id: packet_id })
}

impl MCPacketHeader {
    /// Reads a structured Minecraft packet array, consuming the bytes of
    /// the packet header and returning a parsed packet header object, or
    /// an error.
    /// # Errors
    /// This function may return an `InvalidData` error if the `VarInt` bytes cannot
    /// be properly parsed.
    pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<Self, io::Error> {
        read_packet_header(bytes)
    }
}

pub struct MCPacket {
    pub header: MCPacketHeader,
    pub data: Vec<u8>,
}

impl MCPacket {
    /// Constructs a Minecraft packet object from a set of bytes, consuming the `bytes` passed.
    /// # Errors
    /// This function will return `io::Error` if the bytes cannot be properly parsed.
    pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<MCPacket, io::Error> {
        let header = MCPacketHeader::from_bytes(bytes)?;
        Ok(MCPacket{ header, data: std::mem::take(bytes) })
    }
}
