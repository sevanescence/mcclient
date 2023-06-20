use std::io;

use super::mctypes::{PacketBytesBuilder, VarInt};

pub mod clientbound;
pub mod packet_ids;
pub mod serverbound;

/// Interfaces serverbound packets. Structs implementing this trait are
/// expected to be mcproto-compliant packets; transfering malformatted
/// packets will result in undefined behavior.
pub trait OutboundPacket {
    /// Serializes the internal packet data into an array of bytes.
    fn to_bytes(&self) -> Vec<u8>;
    /// Retrieves the ID of this packet. Largely references compile-time constants.
    fn packet_id(&self) -> i32;
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
    fn from_data(packet: &ClientboundRawPacket) -> Result<Self, io::Error>;

    /// Retrieves the ID of this inbound packet.
    fn packet_id(&self) -> i32;
}

pub struct OutboundPacketBuffer {
    packet_data: Vec<u8>
}

impl From<&dyn OutboundPacket> for OutboundPacketBuffer {
    fn from(value: &dyn OutboundPacket) -> Self {
        OutboundPacketBuffer { packet_data: serialize_packet(value) }
    }
}

impl OutboundPacketBuffer {
    pub fn data(&self) -> &Vec<u8> {
        &self.packet_data
    }
}

/// Serialize a serverbound packet to be sent to a server.
/// Packet structure: https://wiki.vg/Protocol#Packet_format
fn serialize_packet(data: &dyn OutboundPacket) -> Vec<u8> {
    let packet_id_serialized = VarInt::from_i32(data.packet_id());
    let packet_data_bytes = data.to_bytes();
    let packet_size = packet_id_serialized.len() + packet_data_bytes.len() as i32;

    PacketBytesBuilder::new()
        .append_varint(&VarInt::from_i32(packet_size))
        .append_varint(&packet_id_serialized)
        .append_bytes(&packet_data_bytes)
    .build()
}

pub struct MCPacketHeader {
    pub size: i32,
    pub id: i32,
}

/// Attempts to parse a packet header from bytes, consuming the `VarInt` elements of
/// the `Vec<u8>` passed.
/// # Errors
/// This function may return an `InvalidData` error if the `VarInt` bytes cannot be
/// properly parsed.
fn read_packet_header(bytes: &mut Vec<u8>) -> Result<MCPacketHeader, io::Error> {
    let packet_size = VarInt::from_vec_front(bytes)?;
    let packet_id = VarInt::from_vec_front(bytes)?;

    // potential refactor:
    // let packet_size: i32 = mc_types::read_i32_from_varint(&bytes)
    // let bytes_window = bytes[mc_types::varint_size(packet_size)..]
    // let packet_size = mc_types::scan_varint_from_bytes(&bytes);
    // let packet_id = mc_types::scan_varint_from_bytes(&bytes[mc_types::varint_byte_size(packet_size) as usize..]);

    let header = MCPacketHeader {
        size: packet_size.into(),
        id: packet_id.into(),
    };

    Ok(header)
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

// A structured container for a Minecraft network packet. This is primarily
// used to box and parse incoming packets.
pub struct ClientboundRawPacket {
    pub header: MCPacketHeader,
    pub data: Vec<u8>,
}

impl ClientboundRawPacket {
    /// Constructs a Minecraft packet object from a set of bytes, consuming the `bytes` passed.
    /// # Errors
    /// This function will return `io::Error` if the bytes cannot be properly parsed.
    pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<ClientboundRawPacket, io::Error> {
        let header = MCPacketHeader::from_bytes(bytes)?;
        Ok(ClientboundRawPacket {
            header,
            data: std::mem::take(bytes),
        })
    }
}
