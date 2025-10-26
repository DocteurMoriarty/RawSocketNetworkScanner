#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        errors::errors::ParseError,
        utils::convert_bytes::convert_n_to_bytes
    };

    ///////////////////////////////////////////
    ///      Byte Conversion Tests          ///
    ///////////////////////////////////////////

    #[test]
    fn test_convert_u16_success() {
        let value: u16 = 0x1234;
        let result = convert_n_to_bytes(value, 2);
        assert_eq!(result, Ok(vec![0x12, 0x34]));
    }

    #[test]
    fn test_convert_u32_success() {
        let value: u32 = 0xDEADBEEF;
        let result = convert_n_to_bytes(value, 4);
        assert_eq!(result, Ok(vec![0xDE, 0xAD, 0xBE, 0xEF]));
    }

    #[test]
    fn test_convert_u64_success() {
        let value: u64 = 0x0123456789ABCDEF;
        let result = convert_n_to_bytes(value, 8);
        assert_eq!(result, Ok(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]));
    }

    #[test]
    fn test_convert_zero_value() {
        let value: u32 = 0;
        let result = convert_n_to_bytes(value, 4);
        assert_eq!(result, Ok(vec![0x00, 0x00, 0x00, 0x00]));
    }

    #[test]
    fn test_convert_small_numbers() {
        let value: u16 = 0x01;
        let result = convert_n_to_bytes(value, 2);
        assert_eq!(result, Ok(vec![0x00, 0x01]));
    }

    #[test]
    fn test_convert_max_values() {
        // Test u16 max
        let value: u16 = 0xFFFF;
        let result = convert_n_to_bytes(value, 2);
        assert_eq!(result, Ok(vec![0xFF, 0xFF]));

        // Test u32 max
        let value: u32 = 0xFFFFFFFF;
        let result = convert_n_to_bytes(value, 4);
        assert_eq!(result, Ok(vec![0xFF, 0xFF, 0xFF, 0xFF]));
    }

    ///////////////////////////////////////////
    ///      Error Handling Tests           ///
    ///////////////////////////////////////////

    #[test]
    fn test_invalid_size_error() {
        let error = convert_n_to_bytes(0x1234u16, 3);
        assert_eq!(error, Err(ParseError::InvalidLengthBytes { size: 3 }));
    }

    #[test]
    fn test_value_too_large_error() {
        let error = convert_n_to_bytes(0x12345678u32, 2);
        assert_eq!(error, Err(ParseError::ValueTooLarge { 
            value: 0x12345678, 
            size: 2 
        }));
    }

    #[test]
    fn test_zero_size_error() {
        let error = convert_n_to_bytes(0x1234u16, 0);
        assert_eq!(error, Err(ParseError::InvalidLengthBytes { size: 0 }));
    }

    #[test]
    fn test_large_size_error() {
        let error = convert_n_to_bytes(0x1234u16, 10);
        assert_eq!(error, Err(ParseError::InvalidLengthBytes { size: 10 }));
    }

    ///////////////////////////////////////////
    ///      Edge Cases Tests               ///
    ///////////////////////////////////////////

    #[test]
    fn test_single_byte_conversion() {
        let value: u8 = 0xAB;
        let result = convert_n_to_bytes(value, 1);
        assert_eq!(result, Ok(vec![0xAB]));
    }

    #[test]
    fn test_exact_fit_value() {
        // Valeur qui utilise exactement tous les bits disponibles
        let value: u16 = 0xFFFF;
        let result = convert_n_to_bytes(value, 2);
        assert_eq!(result, Ok(vec![0xFF, 0xFF]));
    }

    #[test]
    fn test_minimum_value() {
        let value: u16 = 0x0001;
        let result = convert_n_to_bytes(value, 2);
        assert_eq!(result, Ok(vec![0x00, 0x01]));
    }
}
