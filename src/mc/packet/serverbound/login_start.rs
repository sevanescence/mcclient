use std::mem::size_of;

use uuid::Uuid;

use crate::mc::{mctypes::{MCString, PacketBytesBuilder}, packet::OutboundPacket};

pub struct LoginStart {
    pub username: String,
    pub has_uuid: bool,
    pub uuid: Uuid
}

impl OutboundPacket for LoginStart {
    fn to_bytes(&self) -> Vec<u8> {
        let mut builder = PacketBytesBuilder::new()
            .append_string(self.username.clone())
            .append_bool(self.has_uuid);
        if self.has_uuid {
            builder = builder.append_uuid(&self.uuid);
        }

        builder.byte_buffer
    }

    fn packet_id(&self) -> i32 {
        0x00
    }

    fn len(&self) -> i32 {
        let mut size: i32 = 0;

        size += MCString::from(self.username.clone()).len();
        size += size_of::<bool>() as i32;
        if self.has_uuid {
            size += size_of::<Uuid>() as i32;
        }

        size
    }
}