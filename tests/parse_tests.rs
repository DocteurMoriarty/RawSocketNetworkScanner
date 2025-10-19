#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        parsing::my_parser::{
            parse_mac, 
            parse_ipv4,
            parse_hex
        },
        structs::ipv4::Ipv4Addr,
        errors::errors::ParseError
    };

    ///////////////////////////////////////////
    ///          MAC Parsing Tests          ///
    ///////////////////////////////////////////
    
    #[test]
    fn test_parse_mac_valid() {
        let mac_str = "01:23:45:67:89:ab";
        let expected = [
            0x01,0x23,0x45,0x67,0x89,0xab
        ];
        let result = parse_mac(
            mac_str
        ).unwrap();

        assert_eq!(
            result, 
            expected
        );
    }

    #[test]
    fn test_parse_mac_invalid_length() {
        let mac_str = "01:23:45:67:89";
        let result = parse_mac(
            mac_str
        );
        assert!(
            matches!(
                result, 
                Err(
                    ParseError::InvalidMac
                )
            )
        );
    }

    #[test]
    fn test_parse_mac_invalid_hex() {
        let mac_str = "01:23:45:67:89:zz";
        let result = parse_mac(
            mac_str
        );
        assert!(
            matches!(
                result, 
                Err(
                    ParseError::InvalidHex
                )
            )
        );
    }

    #[test]
    fn test_parse_mac_extra_parts() {
        let mac_str = "01:23:45:67:89:ab:cd";
        let result = parse_mac(
            mac_str
        );
        assert!(
            matches!(
                result, 
                Err(
                    ParseError::InvalidMac
                )
            )
        );
    }

    ///////////////////////////////////////////
    ///         IPv4 Parsing Tests          ///
    ///////////////////////////////////////////
    
    #[test]
    fn test_parse_ipv4_valid() {
        let ip_str = "192.168.1.1";
        let expected = Ipv4Addr { 
            octets: [
                192, 
                168, 
                1, 
                1
            ] 
        };
        let result = parse_ipv4(
            ip_str
        ).unwrap();
        assert_eq!(
            result, 
            expected
        );
    }

    #[test]
    fn test_parse_ipv4_invalid_octet() {
        let ip_str = "192.168.1.256";
        let result = parse_ipv4(
            ip_str
        );
        assert!(
            matches!(
                result, 
                Err(
                    ParseError::InvalidIpv4
                )
            )
        );
    }

    #[test]
    fn test_parse_ipv4_not_enough_octets() {
        let ip_str = "192.168.1";
        let result = parse_ipv4(
            ip_str
        );
        assert!(
            matches!(
                result, 
                Err(
                    ParseError::NotEnoughOctets
                )
            )
        );
    }

    #[test]
    fn test_parse_ipv4_too_many_octets() {
        let ip_str = "192.168.1.1.5";
        let result = parse_ipv4(
            ip_str
        );
        assert!(
            matches!(
                result, 
                Err(
                    ParseError::TooManyOctets
                )
            )
        );
    }

    #[test]
    fn test_parse_hex_valid() {
        let hex_str = "0x1A";
        let result = parse_hex(
            hex_str
        )
        .unwrap();
        assert_eq!(
            result, 
            0x1A
        );
        let hex_str2 = "ff";
        let result2 = parse_hex(
            hex_str2
        )
        .unwrap();
        assert_eq!(
            result2, 
            0xFF
        );
    }

    #[test]
    fn test_parse_hex_invalid() {
        let hex_str = "0xZZ";
        let result = parse_hex(
            hex_str
        );
        assert!(
            matches!(
                result, 
                Err(
                    ParseError::InvalidHex
                )
            )
        );
    }

    #[test]
    fn test_parse_hex_empty() {
        let hex_str = "";
        let result = parse_hex(
            hex_str
        );
        assert!(
            matches!(
                result, 
                Err(
                    ParseError::InvalidHex
                )
            )
        );
    }

    #[test]
    fn test_parse_hex_no_prefix() {
        let hex_str = "1f";
        let result = parse_hex(
            hex_str
        )
        .unwrap();
        assert_eq!(
            result, 
            0x1F
        );
    }


}
