// TODO: Add from_bytes to MCType instead of implementing for individual types uniquely

// TODO: Rewrite MCString to simply build string bytes
//       from a string view.

use crate::mc::mctypes::{MCString, VarInt, MCType};

/// A byte vec builder which helps construct a Minecraft packet that
/// corresponds to the specification of each data type defined by the
/// Minecraft protocol.
pub struct PacketBytesBuilder {
    byte_buffer: Vec<u8>,
}

// TODO: Finish this. Not every data type is supported. NOTE: Might
//       rename some functions to coincide with data type names
//       specified by the Minecraft Protocol docs.
impl PacketBytesBuilder {
    /// Constructs a packet builder with an empty byte vector.
    pub fn new() -> Self {
        PacketBytesBuilder {
            byte_buffer: Vec::<u8>::new(),
        }
    }

    /// Appends a UUID encoded in Big Endian to the buffer.
    pub fn append_uuid(mut self, uuid: &uuid::Uuid) -> Self {
        let uuid_pair = uuid.as_u64_pair();
        self.byte_buffer
            .extend_from_slice(&uuid_pair.1.to_be_bytes());
        self.byte_buffer
            .extend_from_slice(&uuid_pair.0.to_be_bytes());

        self
    }

    // TODO: Why not just make this take a Into<MCString> ? MCStrings end up doing
    //       redundant copies.
    /// Appends a string as a MC-encoded string to the buffer.
    pub fn append_string<S: Into<String>>(mut self, to_string: S) -> Self {
        //let mc_string = mc_types::to_mc_string_bytes(to_string.into());
        let mc_string = MCString::from(to_string.into());
        self.byte_buffer.extend(mc_string.to_bytes());

        self
    }

    /// Appends a bool encoded as a single byte to the buffer.
    pub fn append_bool(mut self, value: bool) -> Self {
        self.byte_buffer.push(value as u8);

        self
    }

    /// Appends a MC-encoded VarInt to the buffer.
    pub fn append_varint(mut self, value: &VarInt) -> Self {
        self.byte_buffer.extend(value.bytes());

        self
    }

    /// Appends a `u16` encoded in Big Endian to the buffer.
    pub fn append_u16(mut self, value: u16) -> Self {
        self.byte_buffer.extend(value.to_be_bytes().to_vec());

        self
    }

    /// Appends `bytes` to the buffer. No additional encoding is made.
    pub fn append_bytes(mut self, bytes: &[u8]) -> Self {
        self.byte_buffer.extend(bytes);

        self
    }

    /// Retrieves the internal buffer of the packet builder.
    /// # Note
    /// This invalidates the internal data, resetting it, and moving the bytes to the caller.
    pub fn build(&mut self) -> Vec<u8> { std::mem::take(&mut self.byte_buffer) }
}
