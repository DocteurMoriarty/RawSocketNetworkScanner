#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        errors::err::ParseError,
        utils::convert_bytes::convert_n_to_bytes
    };

    #[test]
    fn test_u16() {
        let value: u16 = 0x1234;
        let bytes = convert_n_to_bytes(value, 2).unwrap();
        assert_eq!(bytes, vec![0x12, 0x34]);
    }

    #[test]
    fn test_u32() {
        let value: u32 = 0xDEADBEEF;
        let bytes = convert_n_to_bytes(value, 4).unwrap();
        assert_eq!(bytes, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_u64() {
        let value: u64 = 0x0123456789ABCDEF;
        let bytes = convert_n_to_bytes(value, 8).unwrap();
        assert_eq!(bytes, vec![0x01,0x23,0x45,0x67,0x89,0xAB,0xCD,0xEF]);
    }

    #[test]
    fn test_zero() {
        let value: u32 = 0;
        let bytes = convert_n_to_bytes(value, 4).unwrap();
        assert_eq!(bytes, vec![0x00,0x00,0x00,0x00]);
    }

    #[test]
    fn test_small_numbers() {
        let value: u16 = 0x01;
        let bytes = convert_n_to_bytes(value, 2).unwrap();
        assert_eq!(bytes, vec![0x00,0x01]);
    }

    #[test]
    fn test_invalid_size() {
        let err = convert_n_to_bytes(0x1234u16, 3).unwrap_err();
        assert_eq!(err, ParseError::InvalidLengthBytes { size: 3 });
    }

    #[test]
    fn test_value_too_large() {
        let err = convert_n_to_bytes(0x12345678u32, 2).unwrap_err();
        assert_eq!(err, ParseError::ValueTooLarge { value: 0x12345678, size: 2 });
    }

}
