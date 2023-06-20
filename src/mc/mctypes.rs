use std::io;

use serde_json::Value;

// TODO: Add from_bytes to MCType instead of implementing for individual types uniquely

// TODO: Rewrite MCString to simply build string bytes
//       from a string view.

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
    pub fn new() -> Self {
        PacketBytesBuilder {
            byte_buffer: Vec::<u8>::new(),
        }
    }

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
    pub fn append_string<S: Into<String>>(mut self, to_string: S) -> Self {
        //let mc_string = mc_types::to_mc_string_bytes(to_string.into());
        let mc_string = MCString::from(to_string.into());
        self.byte_buffer.extend(mc_string.to_bytes());

        self
    }

    pub fn append_bool(mut self, value: bool) -> Self {
        self.byte_buffer.push(value as u8);

        self
    }

    pub fn append_varint(mut self, value: &VarInt) -> Self {
        self.byte_buffer.extend(value.bytes());

        self
    }

    pub fn append_u16(mut self, value: u16) -> Self {
        self.byte_buffer.extend(value.to_be_bytes().to_vec());

        self
    }

    pub fn append_bytes(mut self, bytes: &[u8]) -> Self {
        self.byte_buffer.extend(bytes);

        self
    }

    pub fn build(&mut self) -> Vec<u8> { std::mem::take(&mut self.byte_buffer) }
}


pub trait MCType: Sized {
    /// Copies the data of this `MCType` and encodes it according to itso
    /// Minecraft protocol packet structure.
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error>;
    /// Gets the bytesize of the serialized version this `MCType`.
    /// # Examples
    /// ```
    /// use crate::mcclient::mc::mctypes::MCString;
    /// use crate::mcclient::mc::mctypes::MCType;
    /// 
    /// let string = MCString::from("Hello!".to_owned());
    /// let size = string.size();
    /// // ^ returns length of "Hello!" + bytesize of `VarInt` size.
    /// // i.e., 6 + [6].len() = 7
    /// ```
    fn size(&self) -> i32;
}

#[allow(dead_code)]
pub struct MCString {
    size: VarInt,
    bytes: Vec<u8>,
}

#[allow(dead_code)]
impl MCString {
    pub fn size(&self) -> &VarInt {
        &self.size
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl From<String> for MCString {
    /// Creates a Minecraft string from a `String`.
    /// # Panics
    /// This function will panic if the size of the String cannot
    /// be parsed to an `i32`.
    fn from(value: String) -> Self {
        let size: i32 = match value.len().try_into() {
            Ok(num) => num,
            Err(msg) => panic!("{}", msg),
        };
        MCString {
            size: VarInt::from(size),
            bytes: value.into_bytes(),
        }
    }
}

impl Into<String> for MCString {
    fn into(self) -> String {
        String::from_utf8(self.bytes).unwrap()
    }
}

impl From<&str> for MCString {
    /// Creates a Minecraft string from a `&str`.
    /// # Panics
    /// This function will panic if the size of the String cannot
    /// be parsed to an `i32`.
    fn from(value: &str) -> Self {
        let size: i32 = match value.len().try_into() {
            Ok(num) => num,
            Err(msg) => panic!("{}", msg),
        };
        MCString {
            size: VarInt::from(size),
            bytes: value.into(),
        }
    }
}

impl MCType for MCString {
    /// Serializes the MCString to a packet-structured array of bytes.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        bytes.append(&mut self.size.to_bytes());
        bytes.append(&mut self.bytes.clone());

        bytes
    }

    /// Attempts to create a `MCString` from a set of bytes, which should be
    /// lead with a `VarInt` descriptor followed by a UTF-8 string.
    /// # Panics
    /// This function will panic if the constituent string cannot properly be
    /// parsed as
    /// # Errors
    /// This function will error in the instance that the `VarInt` header cannot
    /// be parsed.
    fn from_bytes(mut bytes: &[u8]) -> Result<Self, io::Error> {
        let size = VarInt::from_bytes(bytes)?;
        bytes = &bytes[size.len() as usize..];

        let string_res = String::from_utf8(bytes.to_vec());
        if string_res.is_err() {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                string_res.err().unwrap(),
            ))
        } else {
            let string = string_res.unwrap();
            Ok(MCString { size, bytes: string.into_bytes() })
        }
    }

    fn size(&self) -> i32 {
        self.size.len() + TryInto::<i32>::try_into(self.bytes.len()).unwrap()
    }
}

/// A `VarInt` is a variable-length data type encoding a two's
/// complement signed 32-bit integer. A `VarInt` can be anywhere
/// between 1 and 5 bytes. <https://wiki.vg/Protocol#VarInt_and_VarLong>
/// <br>
/// This structure is meant purely for data I/O and should not be used
/// to perform any sort of arithmetic.
#[derive(Clone)]
#[allow(dead_code)]
pub struct VarInt {
    bytes: Vec<u8>,
    value: i32,
}
// Places to remove VarInt dependency:
//  - stream.rs (MinecraftStream::read())
//  - packet/mod.rs

impl From<i32> for VarInt {
    /// Creates a `VarInt` representation of `value`.
    fn from(value: i32) -> Self {
        VarInt {
            bytes: to_varint(value),
            value,
        }
    }
}

impl From<&[u8]> for VarInt {
    /// Creates a `VarInt` from a slice `&[u8]` whose leading bytes represent
    /// a `VarInt` string between 1 and 5 bytes. Because it is expected that the
    /// data being passed to `from_bytes` is a slice of a Minecraft packet, and
    /// the size of a `VarInt` is variadic, a slice larger than the encompassed
    /// number can be passed without unexpected error.
    ///
    /// # Panics
    ///
    /// The parsing of the leading bytes to a `VarInt` will panic if the number
    /// is evaluated to greater than 5 bytes in size. This can be caused by
    /// either the wrong data type being read or the bytes being badly formatted.
    fn from(bytes: &[u8]) -> Self {
        let (val, slice) = match from_varint_bytes(bytes) {
            Ok(t) => t,
            Err(msg) => panic!("{}", msg),
        };
        VarInt {
            bytes: slice.to_vec(),
            value: val,
        }
    }
}

impl MCType for VarInt {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        bytes.append(&mut self.bytes.clone());

        bytes
    }

    /// Similar to VarInt::from for From<&[u8]>, however this is recommended
    /// as it returns an error instead of panicking.
    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let (val, slice) = from_varint_bytes(bytes)?;

        Ok(VarInt {
            bytes: slice.to_vec(),
            value: val,
        })
    }

    fn size(&self) -> i32 {
        self.bytes.len().try_into().unwrap()
    }
}

#[allow(dead_code)]
impl VarInt {
    /// Creats a `VarInt` from a slice of a `Vec<u8>` `vec` and consumes the
    /// front of the `Vec<u8>` that represents the constituent VarInt bytes.
    /// # Errors
    /// This function may error if the head of the vector cannot represent a
    /// `VarInt` type.
    pub fn from_vec_front(vec: &mut Vec<u8>) -> Result<Self, io::Error> {
        let v = VarInt::from_bytes(vec.as_slice())?;
        vec.drain(0..v.len() as usize);
        Ok(v)
    }

    /// Creates a `VarInt` representation of `value`.
    pub fn from_i32(value: i32) -> Self {
        VarInt {
            bytes: to_varint(value),
            value,
        }
    }

    /// Creates a `VarInt` from a slice `&[u8]` whose leading bytes represent
    /// a `VarInt` string between 1 and 5 bytes. Because it is expected that the
    /// data being passed to `from_bytes` is a slice of a Minecraft packet, and
    /// the size of a `VarInt` is variadic, a slice larger than the encompassed
    /// number can be passed without unexpected error.
    ///
    /// # Errors
    ///
    /// The parsing of the leading bytes to a `VarInt` will return an `InvalidData` error
    /// if the number is evaluated to greater than 5 bytes in size. This can be caused
    /// by either the wrong data type being read or the bytes being badly formatted.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let (val, slice) = from_varint_bytes(bytes)?;
        Ok(VarInt {
            bytes: slice.to_vec(),
            value: val,
        })
    }

    /// Retrieves the byte size of the `VarInt`.
    pub fn len(&self) -> i32 {
        self.bytes.len() as i32
    }

    /// Returns a slice of this `VarInt`'s byte array representation.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the numerical equivalent of this `VarInt`.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Sets the value of this `VarInt` to represent the `value` passed. This function
    /// may be used in place of `VarInt::from_i32()` when reinitializing a `VarInt`
    /// is not favorable.
    pub fn set(&mut self, value: i32) {
        self.value = value;
        self.bytes = to_varint(self.value);
    }
}

/// Parses a VarInt from the front of the provided iterator range.
/// # Returns
/// A pair containing the parsed i32 value, and the range excluding the parsed VarInt.
/// # Note
/// This is meant to be used internally.
fn from_varint_bytes(bytes: &[u8]) -> Result<(i32, &[u8]), io::Error> {
    let mut value = 0;
    let mut pos = 0;
    let mut end_idx = 0;

    const SEGMENT_BITS: i32 = 0x7F;
    const CONTINUE_BIT: i32 = 0x80;

    for b in bytes.iter() {
        value |= ((*b as i32) & SEGMENT_BITS) << pos;
        end_idx += 1;

        if (*b as i32) & CONTINUE_BIT == 0 {
            break;
        }

        pos += 7;

        if pos >= 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VarInt descriptor exceeds >5 bytes",
            ));
        }
    }

    Ok((value, &bytes[..end_idx]))
}

/// Parses an i32 to a serialized VarInt byte array.
/// # Returns
/// The serialized VarInt array.
/// # Note
/// This is meant to be used internally.
fn to_varint(mut value: i32) -> Vec<u8> {
    let mut bytes = Vec::<u8>::new();

    const SEGMENT_BITS: i32 = 0x7F;
    const CONTINUE_BIT: i32 = 0x80;

    loop {
        if (value & !SEGMENT_BITS) == 0 {
            bytes.push(value as u8);
            break;
        }

        bytes.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

        // https://stackoverflow.com/a/70212287
        value = ((value as u32) >> 7) as i32;
    }

    bytes
}

#[derive(Debug)]
#[allow(unused)]
pub struct JsonResponse {
    data: Value,
}

impl JsonResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let value: Value = serde_json::from_slice(bytes)?;

        Ok(JsonResponse { data: value })
    }
}
