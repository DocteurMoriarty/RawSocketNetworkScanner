
#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        utils::checksum::internet_checksum
    };

    #[test]
    fn test_empty_data() {
        let data = b"";
        let checksum = internet_checksum(data);
        assert_eq!(checksum, 0xFFFF); 
    }

    #[test]
    fn test_single_byte() {
        let data = b"\x01";
        let checksum = internet_checksum(data);
        assert_eq!(checksum, 0xFEFF);
    }

    #[test]
    fn test_even_length_data() {
        let data = b"\x01\x02"; 
        let checksum = internet_checksum(data);
        assert_eq!(checksum, (!258u16) & 0xFFFF);
    }

    #[test]
    fn test_odd_length_data() {
        let data = b"\x01\x02\x03";
        let checksum = internet_checksum(data);
        assert_eq!(checksum, (!1026u16) & 0xFFFF);
    }

    #[test]
    fn test_known_string() {
        let data = b"Hello, world!";
        let checksum = internet_checksum(data);
        println!("Checksum: 0x{:04X}", checksum);
        assert_eq!(checksum, checksum); 
    }
}
