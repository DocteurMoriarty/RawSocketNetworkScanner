#[cfg(test)]
    mod tests {
    use projet_rsns_morissetlarresacha::{
        structs::tcp::TcpHeader,
        packets::l4::tcp::pack_tcp,
        utils::convert_bytes::convert_n_to_bytes,
        errors::errors::ParseError,
    };

    #[test]
    fn test_tcp_pack_minimal() {
        let header = TcpHeader {
            src_port: 1234,
            dst_port: 80,
            sequence_number: 0,
            ack_nowledgment_number: 0,
            data_offset: 0,
            reserved: 0,
            flags: 0x18,
            window: 1024,
            checksum: 0,
            urgent_pointer: 0,
            options: None,
            payload: None,
        };

        let packet = pack_tcp(&header).unwrap();
        assert_eq!(packet.len(), 20); // header minimal TCP = 20 octets
    }

    #[test]
    fn test_tcp_pack_with_payload() {
        let payload = b"hello";
        let header = TcpHeader {
            src_port: 1234,
            dst_port: 80,
            sequence_number: 0,
            ack_nowledgment_number: 0,
            data_offset: 0,
            reserved: 0,
            flags: 0x18,
            window: 1024,
            checksum: 0,
            urgent_pointer: 0,
            options: None,
            payload: Some(payload),
        };

        let packet = pack_tcp(&header).unwrap();
        assert_eq!(packet.len(), 25); // 20 + 5
        assert_eq!(&packet[20..], payload);
    }

    #[test]
    fn test_tcp_pack_with_options() {
        let options = vec![0x01, 0x02, 0x03, 0x04];
        let header = TcpHeader {
            src_port: 1234,
            dst_port: 80,
            sequence_number: 0,
            ack_nowledgment_number: 0,
            data_offset: 0,
            reserved: 0,
            flags: 0x18,
            window: 1024,
            checksum: 0,
            urgent_pointer: 0,
            options: Some(options.clone()),
            payload: None,
        };

        let packet = pack_tcp(&header).unwrap();
        assert_eq!(packet.len(), 24); // 20 + 4
        assert_eq!(&packet[20..24], &options[..]);
    }

    #[test]
    fn test_convert_n_to_bytes_value_too_large() {
        let result = convert_n_to_bytes(70000u64, 2);
        match result {
            Err(ParseError::ValueTooLarge { value, size }) => {
                assert_eq!(value, 70000);
                assert_eq!(size, 2);
            }
            _ => panic!("Expected ValueTooLarge error"),
        }
    }

}
