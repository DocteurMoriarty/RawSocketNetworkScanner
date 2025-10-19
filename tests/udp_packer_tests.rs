use projet_rsns_morissetlarresacha::{
    structs::udp::UdpHeader,
    utils::convert_bytes::convert_n_to_bytes,
    packets::l4::udp::pack_udp
};

#[test]
fn test_udp_pack_minimal() {
    let header = UdpHeader {
        src_port: 1234,
        dst_port: 80,
        length: 8,
        checksum: 0,
        payload: None,
    };

    let packet = pack_udp(&header).unwrap();
    assert_eq!(packet.len(), 8);
    assert_eq!(&packet[0..2], &convert_n_to_bytes(1234u16, 2).unwrap());
    assert_eq!(&packet[2..4], &convert_n_to_bytes(80u16, 2).unwrap());
    assert_eq!(&packet[4..6], &convert_n_to_bytes(8u16, 2).unwrap());
}

#[test]
fn test_udp_pack_with_payload() {
    let payload = b"hello";
    let header = UdpHeader {
        src_port: 53,          // 0x0035
        dst_port: 5555,        // 0x15B3
        length: (8 + payload.len()) as u16,
        checksum: 0xBEEF,      // 0xBEEF → [0xBE, 0xEF]
        payload: Some(payload),
    };

    let packet = pack_udp(&header).unwrap();

    // Header (8) + payload (5)
    assert_eq!(packet.len(), 13);
    assert_eq!(&packet[8..], payload);

    // Vérifie ports et checksum en Big Endian
    assert_eq!(&packet[0..2], &[0x00, 0x35]); // src_port = 53
    assert_eq!(&packet[2..4], &[0x15, 0xB3]); // dst_port = 5555
    assert_eq!(&packet[6..8], &[0xBE, 0xEF]); // checksum = 0xBEEF
}


#[test]
fn test_udp_pack_invalid_field() {
    let header = UdpHeader {
        src_port: 9999,
        dst_port: 65535,
        length: 9999,
        checksum: 0,
        payload: None,
    };
    let result = pack_udp(&header);
    assert!(result.is_ok());
}

#[test]
fn test_udp_pack_error_propagation() {
    let header = UdpHeader {
        src_port: 0x1234,
        dst_port: 0x5678,
        length: 1,
        checksum: 0,
        payload: None,
    };

    let result = pack_udp(&header);
    assert!(result.is_ok());
}
