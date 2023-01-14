/// A `VarInt` is a variable-length data type encoding a two's
/// complement signed 32-bit integer. A `VarInt` can be anywhere
/// between 1 and 5 bytes. https://wiki.vg/Protocol#VarInt_and_VarLong
/// <br>
/// This structure is meant purely for data I/O and should not be used
/// to perform any sort of arithmetic.
#[derive(Clone)]
pub struct VarInt {
    bytes: Vec<u8>,
    value: i32
}

impl From<i32> for VarInt {
    /// Creates a `VarInt` representation of `value`.
    fn from(value: i32) -> Self {
        VarInt{ bytes: to_varint(value), value }
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
        VarInt{ bytes: bytes.to_vec(), value: from_varint_bytes(bytes) }
    }
}

impl VarInt {
    /// Creates a `VarInt` representation of `value`.
    pub fn from_i32(value: i32) -> Self {
        VarInt{ bytes: to_varint(value), value }
    }

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
    pub fn from_bytes(bytes: &[u8]) -> Self {
        VarInt{ bytes: bytes.to_vec(), value: from_varint_bytes(bytes) }
    }

    /// Retrieves the byte size of the `VarInt`.
    pub fn len(&self) -> i32 {
        self.bytes.len() as i32
    }

    /// Returns a slice of this `VarInt`'s byte array representation.
    pub fn as_bytes(&self) -> &[u8] {
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

fn from_varint_bytes(bytes: &[u8]) -> i32 {
    let mut value = 0;
    let mut pos = 0;

    const SEGMENT_BITS: i32 = 0x7F;
    const CONTINUE_BIT: i32 = 0x80;

    for b in bytes.iter() {
        value |= ((*b as i32) & SEGMENT_BITS) << pos;

        if (*b as i32) & CONTINUE_BIT == 0 {
            break;
        }

        pos += 7;

        if pos >= 32 {
            panic!("VarInt is too big (>5 bytes)");
        }
    }

    value
}

pub fn to_varint(mut value: i32) -> Vec<u8> {
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