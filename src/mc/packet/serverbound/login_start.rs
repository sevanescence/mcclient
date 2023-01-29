use std::mem::size_of;

use uuid::Uuid;

use crate::mc::{mctypes::{MCString, MCType}, packet::OutboundPacket};

pub struct LoginStart {
    pub username: MCString,
    pub has_uuid: bool,
    pub uuid: Uuid
}

impl OutboundPacket for LoginStart {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(self.username.to_bytes());
        bytes.push(self.has_uuid as u8);
        if self.has_uuid {
            let uid = self.uuid.as_u64_pair();
            bytes.extend_from_slice(&uid.1.to_be_bytes());
            bytes.extend_from_slice(&uid.0.to_be_bytes());
        }

        bytes
    }

    fn packet_id(&self) -> i32 {
        0x00
    }

    fn len(&self) -> i32 {
        let mut size: i32 = 0;

        size += self.username.size();
        size += size_of::<bool>() as i32;
        //size += 1; // NO idea why this is needed. Won't work without it.
        if self.has_uuid {
            size += size_of::<Uuid>() as i32;
        }

        size
    }
}