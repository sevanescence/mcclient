#[cfg(test)]
mod tests {
    use crate::mc::{mctypes::{VarInt, MCType, MCString}, packet::{serverbound::{handshake::{Handshake, NextState}, status_request::StatusRequest}, serialize_packet}, connection::PROTOCOL_VERSION};

    #[test]
    fn from_i32_to_varint() {
        assert_eq!(VarInt::from(0).bytes(), [0]);
        assert_eq!(VarInt::from(1).bytes(), [1]);
        assert_eq!(VarInt::from(2).bytes(), [2]);
        assert_eq!(VarInt::from(127).bytes(), [127]);
        assert_eq!(VarInt::from(128).bytes(), [128, 1]);
        assert_eq!(VarInt::from(255).bytes(), [255, 1]);
        assert_eq!(VarInt::from(25565).bytes(), [221, 199, 1]);
        assert_eq!(VarInt::from(2097151).bytes(), [255, 255, 127]);
        assert_eq!(VarInt::from(2147483647).bytes(), [255, 255, 255, 255, 7]);
        assert_eq!(VarInt::from(-1).bytes(), [255, 255, 255, 255, 15]);
        assert_eq!(VarInt::from(-2147483648).bytes(), [128, 128, 128, 128, 8]);
    }

    #[test]
    fn from_varint_to_i32() {
        assert_eq!(0, VarInt::from(&[0][..]).value());
        assert_eq!(1, VarInt::from(&[1][..]).value());
        assert_eq!(2, VarInt::from(&[2][..]).value());
        assert_eq!(127, VarInt::from(&[127][..]).value());
        assert_eq!(128, VarInt::from(&[128, 1][..]).value());
        assert_eq!(255, VarInt::from(&[255, 1][..]).value());
        assert_eq!(25565, VarInt::from(&[221, 199, 1][..]).value());
        assert_eq!(2097151, VarInt::from(&[255, 255, 127][..]).value());
        assert_eq!(2147483647, VarInt::from(&[255, 255, 255, 255, 7][..]).value());
        assert_eq!(-1, VarInt::from(&[255, 255, 255, 255, 15][..]).value());
        assert_eq!(-2147483648, VarInt::from(&[128, 128, 128, 128, 8][..]).value());
    }

    #[test]
    fn from_bytes_to_varint() {
        assert_eq!(&[0][..], VarInt::from(&[0, 0][..]).bytes());
        assert_eq!(&[1][..], VarInt::from(&[1, 0][..]).bytes());
        assert_eq!(&[2][..], VarInt::from(&[2, 0][..]).bytes());
        assert_eq!(&[127][..], VarInt::from(&[127, 0][..]).bytes());
        assert_eq!(&[128, 1][..], VarInt::from(&[128, 1, 0][..]).bytes());
        assert_eq!(&[255, 1][..], VarInt::from(&[255, 1, 0][..]).bytes());
        assert_eq!(&[221, 199, 1][..], VarInt::from(&[221, 199, 1, 0][..]).bytes());
    }

    #[test]
    fn outbound_packet_serialization() {
        let handshake = Handshake {
            protocol_version: VarInt::from(PROTOCOL_VERSION),
            server_addr: MCString::from("localhost"),
            port: 25565,
            next_state: NextState::STATUS
        };

        let actual_data_size = VarInt::from(PROTOCOL_VERSION).len() + 10 + 2 + 1;
        let actual_packet_size = actual_data_size + 1;
        let packet_id: u8 = 0x00;

        let mut fake_packet_bytes = Vec::<u8>::new();
        fake_packet_bytes.append(&mut VarInt::from(actual_packet_size).to_bytes());
        fake_packet_bytes.push(packet_id);
        fake_packet_bytes.append(&mut VarInt::from(PROTOCOL_VERSION).to_bytes());
        fake_packet_bytes.append(&mut VarInt::from("localhost".len() as i32).to_bytes());
        fake_packet_bytes.append(&mut "localhost".as_bytes().to_vec());
        fake_packet_bytes.append(&mut 25565_u16.to_be_bytes().to_vec());
        fake_packet_bytes.append(&mut VarInt::from(NextState::STATUS as i32).to_bytes());

        assert_eq!(serialize_packet(&handshake), fake_packet_bytes);
    }

    #[test]
    fn status_request_packet_serialization() {
        let status_request = StatusRequest;

        assert_eq!(serialize_packet(&status_request), vec![0x01, 0x00]);
    }
}
