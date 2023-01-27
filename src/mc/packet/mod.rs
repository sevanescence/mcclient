use std::io;

use super::mctypes::{VarInt, MCType};

pub mod serverbound;
pub mod clientbound;

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
    /// Attempts to deserialize the given bytes into the implied packet
    /// type. Note: Passing bytes that are not formatted as per the
    /// Minecraft protocol is undefined.
    /// # Returns
    /// This function may error in cases where the packet bytes passed
    /// do not have the same ID as the expected packet. This behavior
    /// is generally undefined, but implementations of clientbound
    /// packets in the internal library necessarily implement this
    /// behavior.
    /// <br> <br>
    /// If the inbound packet data is well-formatted and can be parsed
    /// by the implementing structure, a `Box<Self>` is returned.
    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error>;

    fn from_data(packet: &MCPacket) -> Result<Self, io::Error>;

    /// Gets the size of this packet as a `VarInt`
    fn packet_size(&self) -> VarInt;

    /// Retrieves the ID of this inbound packet.
    fn packet_id(&self) -> i32;
}

/// Serialize a serverbound packet to be sent to a server.
#[allow(dead_code)]
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

/// Attemps to parse a packet header from bytes, consuming the `VarInt` elements of
/// the `Vec<u8>` passed.
/// # Errors
/// This function may return an `InvalidData` error if the `VarInt` bytes cannot be
/// properly parsed.
pub fn read_packet_header(bytes: &mut Vec<u8>) -> Result<MCPacketHeader, io::Error> {
    let packet_size = VarInt::from_vec_front(bytes)?;
    let packet_id = VarInt::from_vec_front(bytes)?;

    Ok(MCPacketHeader{ size: packet_size, id: packet_id })
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
        let header = read_packet_header(bytes)?;
        Ok(MCPacket { header, data: std::mem::take(bytes) })
    }
}
