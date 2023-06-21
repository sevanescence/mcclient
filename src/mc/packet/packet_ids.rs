pub mod serverbound {
    pub const HANDSHAKE_PACKET_ID: i32 =        0x00;
    pub const STATUS_REQUEST: i32 =             0x00;
    pub const LOGIN_START: i32 =                0x00;
}

pub mod clientbound {
    pub const STATUS_RESPONSE: i32 =            0x00;
}
