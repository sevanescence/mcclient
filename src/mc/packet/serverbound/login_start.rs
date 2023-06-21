use uuid::Uuid;

use crate::mc::packet::{OutboundPacket, builder::PacketBytesBuilder, packet_ids};

pub struct LoginStart {
    pub username: String,
    pub has_uuid: bool,
    pub uuid: Uuid,
}

impl OutboundPacket for LoginStart {
    fn to_bytes(&self) -> Vec<u8> {
        let mut builder = PacketBytesBuilder::new()
            .append_string(self.username.clone())
            .append_bool(self.has_uuid);
        if self.has_uuid {
            builder = builder.append_uuid(&self.uuid);
        }

        builder.build()
    }

    fn packet_id(&self) -> i32 {
        packet_ids::serverbound::LOGIN_START
    }
}
