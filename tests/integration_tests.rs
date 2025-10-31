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

        let network_packet = builder.build_packet().unwrap();

        assert_eq!(network_packet.ethernet.src_mac, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(network_packet.ethernet.dst_mac, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        assert_eq!(network_packet.ipv4.src_addr, [192, 168, 1, 100]);
        assert_eq!(network_packet.ipv4.dst_addr, [192, 168, 1, 200]);
        assert_eq!(network_packet.ipv4.protocol, 6); // TCP

        match &network_packet.l4_data {
            L4Data::Tcp(tcp) => {
                assert_eq!(tcp.src_port, 8080);
                assert_eq!(tcp.dst_port, 443);
                assert_eq!(tcp.payload, Some(b"Hello, World!".to_vec()));
            }
            _ => panic!("Expected TCP data"),
        }

        let packet_bytes = network_packet.assemble_packet().unwrap();
        assert!(packet_bytes.len() > 0);

        let calculated_size = network_packet.get_packet_size();
        assert_eq!(packet_bytes.len(), calculated_size);

        let factory = FormatFactory::new();
        let json_data = factory.write_packet(&network_packet, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        assert!(json_str.contains("\"TCP\""));
        assert!(json_str.contains("\"src_port\":8080"));
        assert!(json_str.contains("\"dst_port\":443"));

        let pcap_data = factory.write_packet(&network_packet, FormatType::Pcap).unwrap();
        assert_eq!(pcap_data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]); // Magic number
    }

    #[test]
    fn test_complete_udp_packet_workflow() {
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

        let network_packet = builder.build_packet().unwrap();

        assert_eq!(network_packet.ipv4.protocol, 17);
        match &network_packet.l4_data {
            L4Data::Udp(udp) => {
                assert_eq!(udp.src_port, 53);
                assert_eq!(udp.dst_port, 53);
                assert_eq!(udp.payload, Some(b"DNS Query".to_vec()));
            }
            _ => panic!("Expected UDP data"),
        }

        
        let packet_bytes = network_packet.assemble_packet().unwrap();
        let calculated_size = network_packet.get_packet_size();
        assert_eq!(packet_bytes.len(), calculated_size);

        let factory = FormatFactory::new();
        let json_data = factory.write_packet(&network_packet, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        assert!(json_str.contains("\"UDP\""));
        assert!(json_str.contains("\"src_port\":53"));
    }

    #[test]
    fn test_multiple_packets_workflow() {
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

        let mut packets = Vec::new();
        for builder in &builders {
            packets.push(builder.build_packet().unwrap());
        }

        assert_eq!(packets.len(), 2);

        match &packets[0].l4_data {
            L4Data::Tcp(_) => {},
            _ => panic!("First packet should be TCP"),
        }

        match &packets[1].l4_data {
            L4Data::Udp(_) => {},
            _ => panic!("Second packet should be UDP"),
        }

        let factory = FormatFactory::new();
        let json_data = factory.write_packets(&packets, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        assert!(json_str.contains("\"TCP\""));
        assert!(json_str.contains("\"UDP\""));

        let pcap_data = factory.write_packets(&packets, FormatType::Pcap).unwrap();
        assert_eq!(pcap_data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]);
    }

    #[test]
    fn test_ip_bitfield_application() {
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
        let builder = PacketBuilder::from_cli_args(
            None, 
            None, 
            None, 
            None,
            None,
            None,
            None,
            None,
            None,
        ).unwrap();

        let network_packet = builder.build_packet().unwrap();

        assert_eq!(network_packet.ethernet.src_mac, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(network_packet.ethernet.dst_mac, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(network_packet.ipv4.src_addr, [192, 168, 1, 100]);
        assert_eq!(network_packet.ipv4.dst_addr, [192, 168, 1, 1]);  
        assert_eq!(network_packet.ipv4.protocol, 6);

        match &network_packet.l4_data {
            L4Data::Tcp(tcp) => {
                assert_eq!(tcp.src_port, 12345); 
                assert_eq!(tcp.dst_port, 80);
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
        
        let ethernet_size = 14; 
        let ipv4_size = 20;     
        let tcp_size = 20;      
        let payload_size = 4;
        let expected_total = ethernet_size + ipv4_size + tcp_size + payload_size;
        
        let actual_size = network_packet.get_packet_size();
        assert_eq!(actual_size, expected_total);
        
        let packet_bytes = network_packet.assemble_packet().unwrap();
        assert_eq!(packet_bytes.len(), actual_size);
    }

    #[test]
    fn test_error_handling() {
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

        let result = PacketBuilder::from_cli_args(
            Some("192.168.1.1"),
            Some("192.168.1.2"),
            Some([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x00]),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(result.is_ok());
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

        let json_data = factory.write_packet(&network_packet, FormatType::Json).unwrap();
        let json_str = core::str::from_utf8(&json_data).unwrap();
        
        assert!(json_str.contains("\"src_mac\":\"AA:BB:CC:DD:EE:FF\""));
        assert!(json_str.contains("\"dst_mac\":\"11:22:33:44:55:66\""));
        assert!(json_str.contains("\"src_addr\":\"192.168.1.1\""));
        assert!(json_str.contains("\"dst_addr\":\"192.168.1.2\""));
        assert!(json_str.contains("\"src_port\":8080"));
        assert!(json_str.contains("\"dst_port\":443"));

        let pcap_data = factory.write_packet(&network_packet, FormatType::Pcap).unwrap();
        assert_eq!(pcap_data[0..4], [0xD4, 0xC3, 0xB2, 0xA1]);
        assert!(pcap_data.len() > 24); 
    }
}
