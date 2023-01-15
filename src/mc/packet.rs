use self::clientbound::status_response;

pub mod serverbound;
pub mod clientbound;

/// Interfaces serverbound packets. Structs implementing this trait are
/// expected to be mcproto-compliant packets; transfering malformatted
/// packets will result in undefined behavior.
pub trait OutboundPacket {
    /// Serializes the internal packet data into an array of bytes.
    fn to_vec(&self) -> Vec<u8>;
    /// Retrieves the ID of this packet. This can be a compile-time constant.
    /// Will eventually refactor.
    fn packet_id(&self) -> i32;
    /// Get length of packet (excluding length of Packet ID)
    fn len(&self) -> i32;
}

/// Serialize a serverbound packet to be sent to a server.
pub fn serialize_packet(data: &dyn OutboundPacket) -> Vec<u8> {
    Vec::<u8>::new()
}
