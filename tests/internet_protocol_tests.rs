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
            total_length: 24, // 20 header + 4 payload
            identification: 0x1234,
            flags: 2, // Don't Fragment
            fragment_offset: 0,
            ttl: 64,
            protocol: 6, // TCP
            header_checksum: 0xABCD,
            src_addr: [192, 168, 1, 1],
            dst_addr: [192, 168, 1, 2],
            options: None,
        };

        let payload = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let packet = pack_ipv4(&header, &payload).unwrap();

        // Vérifie la taille
        assert_eq!(packet.len(), 24);

        // Vérifie version + IHL
        assert_eq!(packet[0], 0x45);

        // Vérifie DSCP
        assert_eq!(packet[1], 0x00);

        // Vérifie total_length
        assert_eq!(&packet[2..4], &[0x00, 0x18]);

        // Vérifie identification
        assert_eq!(&packet[4..6], &[0x12, 0x34]);

        // Vérifie flags + fragment offset
        assert_eq!(&packet[6..8], &[0x40, 0x00]);

        // Vérifie TTL
        assert_eq!(packet[8], 64);

        // Vérifie protocole
        assert_eq!(packet[9], 6);

        // Vérifie checksum
        assert_eq!(&packet[10..12], &[0xAB, 0xCD]);

        // Vérifie adresses IP
        assert_eq!(&packet[12..16], &[192, 168, 1, 1]);
        assert_eq!(&packet[16..20], &[192, 168, 1, 2]);

        // Vérifie payload
        assert_eq!(&packet[20..], &payload);
    }

    #[test]
    fn test_pack_ipv4_with_options() {
        let header = Ipv4Header {
            version: 4,
            ihl: 6, // 24 bytes header (20 + 4 options)
            dscp: 0,
            total_length: 28, // 24 header + 4 payload
            identification: 0x5678,
            flags: 0,
            fragment_offset: 0,
            ttl: 128,
            protocol: 17, // UDP
            header_checksum: 0x1234,
            src_addr: [10, 0, 0, 1],
            dst_addr: [10, 0, 0, 2],
            options: Some(vec![0x01, 0x02, 0x03, 0x04]),
        };

        let payload = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let packet = pack_ipv4(&header, &payload).unwrap();

        // Vérifie la taille
        assert_eq!(packet.len(), 28);

        // Vérifie options
        assert_eq!(&packet[20..24], &[0x01, 0x02, 0x03, 0x04]);

        // Vérifie payload
        assert_eq!(&packet[24..], &payload);
    }
}
