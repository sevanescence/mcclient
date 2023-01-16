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
