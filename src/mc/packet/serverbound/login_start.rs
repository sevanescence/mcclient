use uuid::Uuid;

use crate::mc::packet::{OutboundPacket, builder::PacketBytesBuilder, packet_ids};

pub struct LoginStart {
    pub username: String,
    pub uuid: Option<Uuid>,
}

impl OutboundPacket for LoginStart {
    fn to_bytes(&self) -> Vec<u8> {
        let mut builder = PacketBytesBuilder::new()
            .append_string(self.username.clone())
            .append_bool(self.uuid.is_some());
        if self.uuid.is_some() {
            builder = builder.append_uuid(&self.uuid.unwrap());
        }

        builder.build()
    }

    fn packet_id(&self) -> i32 {
        packet_ids::serverbound::LOGIN_START
    }
}
