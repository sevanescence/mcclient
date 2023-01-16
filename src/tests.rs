#[cfg(test)]
mod tests {
    use crate::mc::mctypes::VarInt;

    #[test]
    fn test_from_i32_to_varint() {
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
    fn test_from_varint_to_i32() {
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
}
