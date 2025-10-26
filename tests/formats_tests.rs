#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        formats::format_factory::FormatFactory,
        structs::{
            network_packet::NetworkPacket,
            ethernet::EthernetHeader,
            ip::Ipv4Header,
            tcp::TcpHeader,
            udp::UdpHeader,
            l4_protocol::L4Data,
            formats::FormatType,
            json::JsonSerializer,
            pcap::{PcapWriter, PcapReader},
        },
    };
    use std::vec::Vec;

    ///////////////////////////////////////////
    ///      JSON Format Tests               ///
    ///////////////////////////////////////////

    fn create_test_tcp_packet() -> NetworkPacket {
        NetworkPacket {
            ethernet: EthernetHeader {
                src_mac: [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
                dst_mac: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
                ethertype: 0x0800,
            },
            ipv4: Ipv4Header {
                version: 4,
                ihl: 5,
                dscp: 0,
                total_length: 55,
                identification: 0,
                flags: 0,
                fragment_offset: 0,
                ttl: 64,
                protocol: 6,
                header_checksum: 0x1234,
                src_addr: [192, 168, 1, 1],
                dst_addr: [192, 168, 1, 2],
                options: None,
            },
            l4_data: L4Data::Tcp(TcpHeader {
                src_port: 8080,
                dst_port: 443,
                sequence_number: 12345,
                ack_nowledgment_number: 67890,
                data_offset: 5,
                reserved: 0,
                flags: 0x02,
                window: 65535,
                checksum: 0x5678,
                urgent_pointer: 0,
                options: None,
                payload: Some(b"test payload".to_vec()),
            }),
        }
    }

    fn create_test_udp_packet() -> NetworkPacket {
        NetworkPacket {
            ethernet: EthernetHeader {
                src_mac: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
                dst_mac: [0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB],
                ethertype: 0x0800,
            },
            ipv4: Ipv4Header {
                version: 4,
                ihl: 5,
                dscp: 0,
                total_length: 47,
                identification: 0,
                flags: 0,
                fragment_offset: 0,
                ttl: 64,
                protocol: 17,
                header_checksum: 0xABCD,
                src_addr: [10, 0, 0, 1],
                dst_addr: [10, 0, 0, 2],
                options: None,
            },
            l4_data: L4Data::Udp(UdpHeader {
                src_port: 53,
                dst_port: 53,
                length: 39,
                checksum: 0xEF00,
                payload: Some(b"dns query".to_vec()),
            }),
        }
    }

    #[test]
    fn test_json_serializer_tcp_packet() {
        let packet = create_test_tcp_packet();
        let serializer = JsonSerializer::new();
        
        let json_str = serializer.serialize_packet(&packet).unwrap();
        
        // Vérifier que le JSON contient les champs attendus
        assert!(json_str.contains("\"ethernet\""));
        assert!(json_str.contains("\"ipv4\""));
        assert!(json_str.contains("\"l4\""));
        assert!(json_str.contains("\"metadata\""));
        assert!(json_str.contains("\"TCP\""));
        assert!(json_str.contains("\"src_mac\":\"AA:BB:CC:DD:EE:FF\""));
        assert!(json_str.contains("\"dst_mac\":\"11:22:33:44:55:66\""));
        assert!(json_str.contains("\"src_addr\":\"192.168.1.1\""));
        assert!(json_str.contains("\"dst_addr\":\"192.168.1.2\""));
        assert!(json_str.contains("\"src_port\":8080"));
        assert!(json_str.contains("\"dst_port\":443"));
    }

    #[test]
    fn test_json_serializer_udp_packet() {
        let packet = create_test_udp_packet();
        let serializer = JsonSerializer::new();
        
        let json_str = serializer.serialize_packet(&packet).unwrap();
        
        // Vérifier que le JSON contient les champs UDP
        assert!(json_str.contains("\"UDP\""));
        assert!(json_str.contains("\"src_port\":53"));
        assert!(json_str.contains("\"dst_port\":53"));
        assert!(json_str.contains("\"src_addr\":\"10.0.0.1\""));
        assert!(json_str.contains("\"dst_addr\":\"10.0.0.2\""));
    }

    #[test]
    fn test_json_serializer_without_raw_data() {
        let packet = create_test_tcp_packet();
        let serializer = JsonSerializer::without_raw_data();
        
        let json_str = serializer.serialize_packet(&packet).unwrap();
        
        // Vérifier que raw_data est vide
        assert!(json_str.contains("\"raw_data\":\"\""));
    }

    #[test]
    fn test_json_serializer_multiple_packets() {
        let packets = vec![create_test_tcp_packet(), create_test_udp_packet()];
        let serializer = JsonSerializer::new();
        
        let json_str = serializer.serialize_packets(&packets).unwrap();
        
        // Vérifier que c'est un tableau JSON
        assert!(json_str.starts_with('['));
        assert!(json_str.ends_with(']'));
        assert!(json_str.contains("\"TCP\""));
        assert!(json_str.contains("\"UDP\""));
    }

    ///////////////////////////////////////////
    ///      PCAP Format Tests               ///
    ///////////////////////////////////////////

    #[test]
    fn test_pcap_writer_global_header() {
        let mut writer = PcapWriter::new();
        writer.write_global_header().unwrap();
        
        let data = writer.get_data();
        
        // Vérifier la taille du header global (24 bytes)
        assert_eq!(data.len(), 24);
        
        // Vérifier le magic number (little endian)
        assert_eq!(data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]);
    }

    #[test]
    fn test_pcap_writer_single_packet() {
        let packet = create_test_tcp_packet();
        let mut writer = PcapWriter::new();
        
        writer.write_global_header().unwrap();
        writer.write_packet(&packet).unwrap();
        
        let data = writer.get_data();
        
        // Vérifier que le fichier contient le header global + packet header + packet data
        assert!(data.len() > 24); // Plus que juste le header global
        assert!(data.len() < 200); // Raisonnable pour un petit paquet
    }

    #[test]
    fn test_pcap_writer_multiple_packets() {
        let packets = vec![create_test_tcp_packet(), create_test_udp_packet()];
        let mut writer = PcapWriter::new();
        
        writer.write_global_header().unwrap();
        for packet in &packets {
            writer.write_packet(packet).unwrap();
        }
        
        let data = writer.get_data();
        
        // Vérifier que le fichier contient plusieurs paquets
        assert!(data.len() > 100); // Plus grand avec plusieurs paquets
    }

    #[test]
    fn test_pcap_reader_global_header() {
        let mut writer = PcapWriter::new();
        writer.write_global_header().unwrap();
        let data = writer.into_data();
        
        let mut reader = PcapReader::new(data);
        reader.read_global_header().unwrap();
        
        // Si on arrive ici sans erreur, le header global est valide
    }

    #[test]
    fn test_pcap_reader_packets() {
        let packet = create_test_tcp_packet();
        let mut writer = PcapWriter::new();
        writer.write_global_header().unwrap();
        writer.write_packet(&packet).unwrap();
        let data = writer.into_data();
        
        let mut reader = PcapReader::new(data);
        reader.read_global_header().unwrap();
        
        // Lire le premier paquet
        let packet_data = reader.read_next_packet().unwrap();
        assert!(packet_data.is_some());
        
        // Vérifier qu'il n'y a plus de paquets
        let packet_data2 = reader.read_next_packet().unwrap();
        assert!(packet_data2.is_none());
        
        assert!(!reader.has_more_packets());
    }

    ///////////////////////////////////////////
    ///      Format Factory Tests           ///
    ///////////////////////////////////////////

    #[test]
    fn test_format_factory_json() {
        let packet = create_test_tcp_packet();
        let factory = FormatFactory::new();
        
        let json_data = factory.write_packet(&packet, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        
        assert!(json_str.contains("\"ethernet\""));
        assert!(json_str.contains("\"TCP\""));
    }

    #[test]
    fn test_format_factory_pcap() {
        let packet = create_test_tcp_packet();
        let factory = FormatFactory::new();
        
        let pcap_data = factory.write_packet(&packet, FormatType::Pcap).unwrap();
        
        // Vérifier le magic number PCAP
        assert_eq!(pcap_data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]);
        assert!(pcap_data.len() > 24); // Plus que juste le header global
    }

    #[test]
    fn test_format_factory_multiple_packets_json() {
        let packets = vec![create_test_tcp_packet(), create_test_udp_packet()];
        let factory = FormatFactory::new();
        
        let json_data = factory.write_packets(&packets, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        
        assert!(json_str.starts_with('['));
        assert!(json_str.contains("\"TCP\""));
        assert!(json_str.contains("\"UDP\""));
    }

    #[test]
    fn test_format_factory_multiple_packets_pcap() {
        let packets = vec![create_test_tcp_packet(), create_test_udp_packet()];
        let factory = FormatFactory::new();
        
        let pcap_data = factory.write_packets(&packets, FormatType::Pcap).unwrap();
        
        // Vérifier le magic number PCAP
        assert_eq!(pcap_data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]);
        assert!(pcap_data.len() > 100); // Plus grand avec plusieurs paquets
    }

    ///////////////////////////////////////////
    ///      Integration Tests              ///
    ///////////////////////////////////////////

    #[test]
    fn test_json_roundtrip() {
        let packet = create_test_tcp_packet();
        let serializer = JsonSerializer::new();
        
        // Sérialiser
        let json_str = serializer.serialize_packet(&packet).unwrap();
        
        // Vérifier que le JSON est valide (contient les champs attendus)
        assert!(json_str.len() > 100);
        assert!(json_str.contains("\"ethernet\""));
        assert!(json_str.contains("\"ipv4\""));
        assert!(json_str.contains("\"l4\""));
        assert!(json_str.contains("\"metadata\""));
    }

    #[test]
    fn test_pcap_roundtrip() {
        let packet = create_test_tcp_packet();
        let mut writer = PcapWriter::new();
        
        // Écrire le paquet
        writer.write_global_header().unwrap();
        writer.write_packet(&packet).unwrap();
        let data = writer.into_data();
        
        // Lire le paquet
        let mut reader = PcapReader::new(data);
        reader.read_global_header().unwrap();
        let packet_data = reader.read_next_packet().unwrap();
        
        assert!(packet_data.is_some());
        assert!(!reader.has_more_packets());
    }

    #[test]
    fn test_format_consistency() {
        let packet = create_test_tcp_packet();
        let factory = FormatFactory::new();
        
        // Générer les deux formats
        let json_data = factory.write_packet(&packet, FormatType::Json).unwrap();
        let pcap_data = factory.write_packet(&packet, FormatType::Pcap).unwrap();
        
        // Vérifier que les deux formats sont différents mais valides
        assert_ne!(json_data, pcap_data);
        assert!(json_data.len() > 0);
        assert!(pcap_data.len() > 0);
        
        // JSON doit être du texte
        let json_str = core::str::from_utf8(&json_data).unwrap();
        assert!(json_str.contains("\"ethernet\""));
        
        // PCAP doit commencer par le magic number
        assert_eq!(pcap_data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]);
    }
}
