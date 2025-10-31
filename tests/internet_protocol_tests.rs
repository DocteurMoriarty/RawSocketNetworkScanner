#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        structs::ip::Ipv4Header,
        packets::ip::pack_ipv4,
    };

    #[test]
    fn test_pack_ipv4_basic() {
        let header = Ipv4Header {
            version: 4,
            ihl: 5,
            dscp: 0,
            total_length: 24, 
            identification: 0x1234,
            flags: 2,
            fragment_offset: 0,
            ttl: 64,
            protocol: 6,
            header_checksum: 0xABCD,
            src_addr: [192, 168, 1, 1],
            dst_addr: [192, 168, 1, 2],
            options: None,
        };

        let payload = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let packet = pack_ipv4(&header, &payload).unwrap();

        assert_eq!(packet.len(), 24);
        assert_eq!(packet[0], 0x45);
        assert_eq!(packet[1], 0x00);
        assert_eq!(&packet[2..4], &[0x00, 0x18]);
        assert_eq!(&packet[4..6], &[0x12, 0x34]);
        assert_eq!(&packet[6..8], &[0x40, 0x00]);
        assert_eq!(packet[8], 64);
        assert_eq!(packet[9], 6);
        assert_eq!(&packet[10..12], &[0xAB, 0xCD]);
        assert_eq!(&packet[12..16], &[192, 168, 1, 1]);
        assert_eq!(&packet[16..20], &[192, 168, 1, 2]);
        assert_eq!(&packet[20..], &payload);
    }

    #[test]
    fn test_pack_ipv4_with_options() {
        let header = Ipv4Header {
            version: 4,
            ihl: 6, 
            dscp: 0,
            total_length: 28,
            identification: 0x5678,
            flags: 0,
            fragment_offset: 0,
            ttl: 128,
            protocol: 17,
            header_checksum: 0x1234,
            src_addr: [10, 0, 0, 1],
            dst_addr: [10, 0, 0, 2],
            options: Some(vec![0x01, 0x02, 0x03, 0x04]),
        };

        let payload = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let packet = pack_ipv4(&header, &payload).unwrap();

        assert_eq!(packet.len(), 28);
        assert_eq!(&packet[20..24], &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(&packet[24..], &payload);
    }
}
