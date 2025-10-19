#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        errors::errors::ParseError,
        utils::convert_bytes::convert_n_to_bytes
    };

    #[test]
    fn test_convert_u16_success() {
        let value: u16 = 0x1234;
        let result = convert_n_to_bytes(
            value, 
            2
        );
        assert_eq!(
            result, 
            Ok(
                vec![
                    0x12, 0x34
                ]
            )
        );
    }

    #[test]
    fn test_u32() {
        let value: u32 = 0xDEADBEEF;
        let result = convert_n_to_bytes(
            value, 
            4
        );
        assert_eq!(
            result, 
            Ok(
                vec![
                    0xDE, 0xAD, 0xBE, 0xEF
                ]
            )
        );
    }

    #[test]
    fn test_u64() {
        let value: u64 = 0x0123456789ABCDEF;
        let result = convert_n_to_bytes(value, 8);
        assert_eq!(
            result, 
            Ok(
                vec![
                    0x01,0x23,0x45,0x67,0x89,0xAB,0xCD,0xEF
                ]
            )
        );
    }

    #[test]
    fn test_zero() {
        let value: u32 = 0;
        let result = convert_n_to_bytes(
            value, 
            4
        );
        assert_eq!(
            result,
            Ok(
                vec![
                    0x00,0x00,0x00,0x00
                ]
            )
        );
    }

    #[test]
    fn test_small_numbers() {
        let value: u16 = 0x01;
        let result = convert_n_to_bytes(
            value, 
            2
        );
        assert_eq!(
            result, 
            Ok(
                vec![
                    0x00,0x01
                ]
            )
        );
    }

    #[test]
    fn test_invalid_size() {
        let error = convert_n_to_bytes(
            0x1234u16, 
            3
        );
        assert_eq!(
            error, 
            Err(
                ParseError::InvalidLengthBytes { 
                    size: 3 
                }
            )
        );
    }

    #[test]
    fn test_value_too_large() {
        let error = convert_n_to_bytes(
            0x12345678u32, 
            2
        );
        assert_eq!(
            error, 
            Err(
                ParseError::ValueTooLarge { 
                    value: 0x12345678, 
                    size: 2 
                }
            )
        );
    }
}
