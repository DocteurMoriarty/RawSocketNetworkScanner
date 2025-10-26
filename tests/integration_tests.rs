#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        structs::{
            packet_builder::PacketBuilder,
            l4_protocol::L4Data,
        },
        formats::format_factory::FormatFactory,
        structs::formats::FormatType,
    };
    use std::vec::Vec;

    ///////////////////////////////////////////
    ///      Integration Tests              ///
    ///////////////////////////////////////////

    #[test]
    fn test_complete_tcp_packet_workflow() {
        // 1. Créer un PacketBuilder avec tous les paramètres
        let builder = PacketBuilder::from_cli_args(
            Some("192.168.1.100"),
            Some("192.168.1.200"),
            Some([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
            Some([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]),
            Some(8080),
            Some(443),
            Some("tcp"),
            Some(0x04),
            Some(b"Hello, World!".to_vec()),
        ).unwrap();

        // 2. Construire le paquet réseau complet
        let network_packet = builder.build_packet().unwrap();

        // 3. Vérifier la structure du paquet
        assert_eq!(network_packet.ethernet.src_mac, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(network_packet.ethernet.dst_mac, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        assert_eq!(network_packet.ipv4.src_addr, [192, 168, 1, 100]);
        assert_eq!(network_packet.ipv4.dst_addr, [192, 168, 1, 200]);
        assert_eq!(network_packet.ipv4.protocol, 6); // TCP

        // 4. Vérifier les données TCP
        match &network_packet.l4_data {
            L4Data::Tcp(tcp) => {
                assert_eq!(tcp.src_port, 8080);
                assert_eq!(tcp.dst_port, 443);
                assert_eq!(tcp.payload, Some(b"Hello, World!".to_vec()));
            }
            _ => panic!("Expected TCP data"),
        }

        // 5. Assembler le paquet en bytes
        let packet_bytes = network_packet.assemble_packet().unwrap();
        assert!(packet_bytes.len() > 0);

        // 6. Vérifier la taille calculée
        let calculated_size = network_packet.get_packet_size();
        assert_eq!(packet_bytes.len(), calculated_size);

        // 7. Sérialiser en JSON
        let factory = FormatFactory::new();
        let json_data = factory.write_packet(&network_packet, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        assert!(json_str.contains("\"TCP\""));
        assert!(json_str.contains("\"src_port\":8080"));
        assert!(json_str.contains("\"dst_port\":443"));

        // 8. Sérialiser en PCAP
        let pcap_data = factory.write_packet(&network_packet, FormatType::Pcap).unwrap();
        assert_eq!(pcap_data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]); // Magic number
    }

    #[test]
    fn test_complete_udp_packet_workflow() {
        // 1. Créer un PacketBuilder UDP
        let builder = PacketBuilder::from_cli_args(
            Some("10.0.0.1"),
            Some("10.0.0.2"),
            Some([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]),
            Some([0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB]),
            Some(53),
            Some(53),
            Some("udp"),
            Some(0x00),
            Some(b"DNS Query".to_vec()),
        ).unwrap();

        // 2. Construire le paquet réseau
        let network_packet = builder.build_packet().unwrap();

        // 3. Vérifier la structure UDP
        assert_eq!(network_packet.ipv4.protocol, 17); // UDP
        match &network_packet.l4_data {
            L4Data::Udp(udp) => {
                assert_eq!(udp.src_port, 53);
                assert_eq!(udp.dst_port, 53);
                assert_eq!(udp.payload, Some(b"DNS Query".to_vec()));
            }
            _ => panic!("Expected UDP data"),
        }

        // 4. Assembler et vérifier
        let packet_bytes = network_packet.assemble_packet().unwrap();
        let calculated_size = network_packet.get_packet_size();
        assert_eq!(packet_bytes.len(), calculated_size);

        // 5. Formats de sortie
        let factory = FormatFactory::new();
        let json_data = factory.write_packet(&network_packet, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        assert!(json_str.contains("\"UDP\""));
        assert!(json_str.contains("\"src_port\":53"));
    }

    #[test]
    fn test_multiple_packets_workflow() {
        // Créer plusieurs paquets différents
        let builders = vec![
            PacketBuilder::from_cli_args(
                Some("192.168.1.1"),
                Some("192.168.1.2"),
                None,
                None,
                Some(80),
                Some(8080),
                Some("tcp"),
                Some(0x00),
                Some(b"HTTP Request".to_vec()),
            ).unwrap(),
            PacketBuilder::from_cli_args(
                Some("192.168.1.3"),
                Some("192.168.1.4"),
                None,
                None,
                Some(53),
                Some(53),
                Some("udp"),
                Some(0x04),
                Some(b"DNS Query".to_vec()),
            ).unwrap(),
        ];

        // Construire tous les paquets
        let mut packets = Vec::new();
        for builder in &builders {
            packets.push(builder.build_packet().unwrap());
        }

        // Vérifier qu'on a bien 2 paquets
        assert_eq!(packets.len(), 2);

        // Vérifier les types de protocoles
        match &packets[0].l4_data {
            L4Data::Tcp(_) => {},
            _ => panic!("First packet should be TCP"),
        }

        match &packets[1].l4_data {
            L4Data::Udp(_) => {},
            _ => panic!("Second packet should be UDP"),
        }

        // Sérialiser en JSON multiple
        let factory = FormatFactory::new();
        let json_data = factory.write_packets(&packets, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        assert!(json_str.contains("\"TCP\""));
        assert!(json_str.contains("\"UDP\""));

        // Sérialiser en PCAP multiple
        let pcap_data = factory.write_packets(&packets, FormatType::Pcap).unwrap();
        assert_eq!(pcap_data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]);
    }

    #[test]
    fn test_ip_bitfield_application() {
        // Test avec différents bitfields
        let bitfields = [0x00, 0x04, 0x08, 0x0C];
        
        for bitfield in &bitfields {
            let builder = PacketBuilder::from_cli_args(
                Some("192.168.1.1"),
                Some("192.168.1.2"),
                None,
                None,
                None,
                None,
                Some("tcp"),
                Some(*bitfield),
                None,
            ).unwrap();

            let network_packet = builder.build_packet().unwrap();
            
            // Vérifier que le bitfield est appliqué
            assert_eq!(network_packet.ipv4.flags, (bitfield >> 5) & 0x07);
            assert_eq!(network_packet.ipv4.fragment_offset, ((bitfield & 0x1F) as u16) << 8);
        }
    }

    #[test]
    fn test_checksum_calculation() {
        let builder = PacketBuilder::from_cli_args(
            Some("192.168.1.1"),
            Some("192.168.1.2"),
            None,
            None,
            Some(8080),
            Some(443),
            Some("tcp"),
            Some(0x00),
            Some(b"test payload".to_vec()),
        ).unwrap();

        let network_packet = builder.build_packet().unwrap();

        // Vérifier que les checksums sont calculés
        assert!(network_packet.ipv4.header_checksum > 0);
        
        match &network_packet.l4_data {
            L4Data::Tcp(tcp) => {
                assert!(tcp.checksum > 0);
            }
            _ => panic!("Expected TCP data"),
        }
    }

    #[test]
    fn test_default_values() {
        // Test avec des valeurs minimales
        let builder = PacketBuilder::from_cli_args(
            None, // src_ip par défaut
            None, // dst_ip par défaut
            None, // src_mac par défaut
            None, // dst_mac par défaut
            None, // src_port par défaut
            None, // dst_port par défaut
            None, // protocol par défaut (TCP)
            None, // bitfield par défaut
            None, // payload par défaut
        ).unwrap();

        let network_packet = builder.build_packet().unwrap();

        // Vérifier les valeurs par défaut
        assert_eq!(network_packet.ethernet.src_mac, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(network_packet.ethernet.dst_mac, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(network_packet.ipv4.src_addr, [192, 168, 1, 100]); // Valeur par défaut
        assert_eq!(network_packet.ipv4.dst_addr, [192, 168, 1, 1]);   // Valeur par défaut
        assert_eq!(network_packet.ipv4.protocol, 6); // TCP par défaut

        match &network_packet.l4_data {
            L4Data::Tcp(tcp) => {
                assert_eq!(tcp.src_port, 12345); // Port par défaut
                assert_eq!(tcp.dst_port, 80);    // Port par défaut
            }
            _ => panic!("Expected TCP data"),
        }
    }

    #[test]
    fn test_packet_size_consistency() {
        let builder = PacketBuilder::from_cli_args(
            Some("192.168.1.1"),
            Some("192.168.1.2"),
            None,
            None,
            Some(8080),
            Some(443),
            Some("tcp"),
            Some(0x00),
            Some(b"test".to_vec()),
        ).unwrap();

        let network_packet = builder.build_packet().unwrap();
        
        // Calculer la taille manuellement
        let ethernet_size = 14; // Ethernet header
        let ipv4_size = 20;     // IPv4 header
        let tcp_size = 20;       // TCP header
        let payload_size = 4;    // "test"
        let expected_total = ethernet_size + ipv4_size + tcp_size + payload_size;
        
        let actual_size = network_packet.get_packet_size();
        assert_eq!(actual_size, expected_total);
        
        let packet_bytes = network_packet.assemble_packet().unwrap();
        assert_eq!(packet_bytes.len(), actual_size);
    }

    #[test]
    fn test_error_handling() {
        // Test avec des adresses IP invalides
        let result = PacketBuilder::from_cli_args(
            Some("invalid_ip"),
            Some("192.168.1.2"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(result.is_err());

        // Test avec des adresses MAC valides (pas d'erreur attendue)
        let result = PacketBuilder::from_cli_args(
            Some("192.168.1.1"),
            Some("192.168.1.2"),
            Some([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x00]), // MAC valide
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(result.is_ok()); // Cette fois c'est OK car la MAC est valide
    }

    #[test]
    fn test_format_roundtrip() {
        let builder = PacketBuilder::from_cli_args(
            Some("192.168.1.1"),
            Some("192.168.1.2"),
            Some([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
            Some([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]),
            Some(8080),
            Some(443),
            Some("tcp"),
            Some(0x04),
            Some(b"roundtrip test".to_vec()),
        ).unwrap();

        let network_packet = builder.build_packet().unwrap();
        let factory = FormatFactory::new();

        // Test JSON roundtrip
        let json_data = factory.write_packet(&network_packet, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        
        // Vérifier que le JSON contient les bonnes informations
        assert!(json_str.contains("\"src_mac\":\"AA:BB:CC:DD:EE:FF\""));
        assert!(json_str.contains("\"dst_mac\":\"11:22:33:44:55:66\""));
        assert!(json_str.contains("\"src_addr\":\"192.168.1.1\""));
        assert!(json_str.contains("\"dst_addr\":\"192.168.1.2\""));
        assert!(json_str.contains("\"src_port\":8080"));
        assert!(json_str.contains("\"dst_port\":443"));

        // Test PCAP roundtrip
        let pcap_data = factory.write_packet(&network_packet, FormatType::Pcap).unwrap();
        assert_eq!(pcap_data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]); // Magic number
        assert!(pcap_data.len() > 24); // Plus que juste le header global
    }
}
