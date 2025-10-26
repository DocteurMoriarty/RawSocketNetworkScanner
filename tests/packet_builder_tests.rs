
#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        packets::builder::{
            packet_factory::PacketFactory,
            packet_assembler::PacketAssembler,
            tcp_builder::TcpBuilder,
            udp_builder::UdpBuilder,
            ipv4_builder::Ipv4Builder,
            ethernet_builder::EthernetBuilder,
        },
        structs::{
            packet_builder::PacketBuilder,
            l4_protocol::{L4Protocol, L4Data},
            ipv4::Ipv4Addr,
            network_packet::NetworkPacket,
            tcp::TcpHeader,
        },
    };
    use std::vec::Vec;

    ///////////////////////////////////////////
    ///      PacketBuilder Tests             ///
    ///////////////////////////////////////////

    #[test]
    fn test_packet_builder_from_cli_args_complete() {
        let builder = PacketBuilder::from_cli_args(
            Some("192.168.1.1"),
            Some("192.168.1.2"),
            Some([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
            Some([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]),
            Some(8080),
            Some(443),
            Some("tcp"),
            Some(0x04),
            Some(b"test payload".to_vec()),
        ).unwrap();

        assert_eq!(builder.src_ip.octets, [192, 168, 1, 1]);
        assert_eq!(builder.dst_ip.octets, [192, 168, 1, 2]);
        assert_eq!(builder.src_mac, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(builder.dst_mac, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        assert_eq!(builder.src_port, 8080);
        assert_eq!(builder.dst_port, 443);
        assert_eq!(builder.protocol, L4Protocol::Tcp);
        assert_eq!(builder.ip_bitfield, 0x04);
        assert_eq!(builder.payload, Some(b"test payload".to_vec()));
    }

    #[test]
    fn test_packet_builder_from_cli_args_minimal() {
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

        assert_eq!(builder.src_ip.octets, [192, 168, 1, 100]); // Valeur par défaut
        assert_eq!(builder.dst_ip.octets, [192, 168, 1, 1]);   // Valeur par défaut
        assert_eq!(builder.src_mac, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(builder.dst_mac, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(builder.src_port, 12345);
        assert_eq!(builder.dst_port, 80);
        assert_eq!(builder.protocol, L4Protocol::Tcp);
        assert_eq!(builder.ip_bitfield, 0x00);
        assert_eq!(builder.payload, None);
    }

    #[test]
    fn test_packet_builder_udp_protocol() {
        let builder = PacketBuilder::from_cli_args(
            Some("10.0.0.1"),
            Some("10.0.0.2"),
            None,
            None,
            None,
            None,
            Some("udp"),
            None,
            None,
        ).unwrap();

        assert_eq!(builder.protocol, L4Protocol::Udp);
    }

    ///////////////////////////////////////////
    ///      PacketFactory Tests            ///
    ///////////////////////////////////////////

    #[test]
    fn test_packet_factory_build_tcp_packet() {
        let factory = PacketFactory::new(
            Ipv4Addr { octets: [192, 168, 1, 1] },
            Ipv4Addr { octets: [192, 168, 1, 2] },
            0x00,
        );

        let builder = PacketBuilder {
            src_ip: Ipv4Addr { octets: [192, 168, 1, 1] },
            dst_ip: Ipv4Addr { octets: [192, 168, 1, 2] },
            src_mac: [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
            dst_mac: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
            src_port: 8080,
            dst_port: 443,
            protocol: L4Protocol::Tcp,
            ip_bitfield: 0x00,
            payload: Some(b"test".to_vec()),
        };

        let packet = factory.build_packet(&builder).unwrap();

        // Vérifier Ethernet
        assert_eq!(packet.ethernet.src_mac, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(packet.ethernet.dst_mac, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        assert_eq!(packet.ethernet.ethertype, 0x0800);

        // Vérifier IPv4
        assert_eq!(packet.ipv4.src_addr, [192, 168, 1, 1]);
        assert_eq!(packet.ipv4.dst_addr, [192, 168, 1, 2]);
        assert_eq!(packet.ipv4.protocol, 6); // TCP

        // Vérifier TCP
        match &packet.l4_data {
            L4Data::Tcp(tcp) => {
                assert_eq!(tcp.src_port, 8080);
                assert_eq!(tcp.dst_port, 443);
                assert_eq!(tcp.payload, Some(b"test".to_vec()));
            }
            _ => panic!("Expected TCP data"),
        }
    }

    #[test]
    fn test_packet_factory_build_udp_packet() {
        let factory = PacketFactory::new(
            Ipv4Addr { octets: [10, 0, 0, 1] },
            Ipv4Addr { octets: [10, 0, 0, 2] },
            0x04,
        );

        let builder = PacketBuilder {
            src_ip: Ipv4Addr { octets: [10, 0, 0, 1] },
            dst_ip: Ipv4Addr { octets: [10, 0, 0, 2] },
            src_mac: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            dst_mac: [0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB],
            src_port: 53,
            dst_port: 53,
            protocol: L4Protocol::Udp,
            ip_bitfield: 0x04,
            payload: Some(b"dns query".to_vec()),
        };

        let packet = factory.build_packet(&builder).unwrap();

        // Vérifier IPv4 avec bitfield
        assert_eq!(packet.ipv4.protocol, 17); // UDP
        assert_eq!(packet.ipv4.flags, 0); // Bitfield appliqué

        // Vérifier UDP
        match &packet.l4_data {
            L4Data::Udp(udp) => {
                assert_eq!(udp.src_port, 53);
                assert_eq!(udp.dst_port, 53);
                assert_eq!(udp.payload, Some(b"dns query".to_vec()));
            }
            _ => panic!("Expected UDP data"),
        }
    }

    ///////////////////////////////////////////
    ///      PacketAssembler Tests           ///
    ///////////////////////////////////////////

    #[test]
    fn test_packet_assembler_tcp_packet() {
        let factory = PacketFactory::new(
            Ipv4Addr { octets: [192, 168, 1, 1] },
            Ipv4Addr { octets: [192, 168, 1, 2] },
            0x00,
        );

        let builder = PacketBuilder {
            src_ip: Ipv4Addr { octets: [192, 168, 1, 1] },
            dst_ip: Ipv4Addr { octets: [192, 168, 1, 2] },
            src_mac: [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
            dst_mac: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
            src_port: 8080,
            dst_port: 443,
            protocol: L4Protocol::Tcp,
            ip_bitfield: 0x00,
            payload: Some(b"hello".to_vec()),
        };

        let packet = factory.build_packet(&builder).unwrap();
        let assembler = PacketAssembler::new();
        
        let packet_bytes = assembler.assemble_packet(&packet).unwrap();
        let packet_size = assembler.get_packet_size(&packet);

        // Vérifier la taille
        assert_eq!(packet_bytes.len(), packet_size);
        assert!(packet_bytes.len() > 0);

        // Vérifier les premiers octets (Ethernet header)
        assert_eq!(packet_bytes[0..6], [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]); // dst_mac
        assert_eq!(packet_bytes[6..12], [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]); // src_mac
        assert_eq!(packet_bytes[12..14], [0x08, 0x00]); // ethertype
    }

    #[test]
    fn test_packet_assembler_udp_packet() {
        let factory = PacketFactory::new(
            Ipv4Addr { octets: [10, 0, 0, 1] },
            Ipv4Addr { octets: [10, 0, 0, 2] },
            0x00,
        );

        let builder = PacketBuilder {
            src_ip: Ipv4Addr { octets: [10, 0, 0, 1] },
            dst_ip: Ipv4Addr { octets: [10, 0, 0, 2] },
            src_mac: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            dst_mac: [0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB],
            src_port: 53,
            dst_port: 53,
            protocol: L4Protocol::Udp,
            ip_bitfield: 0x00,
            payload: Some(b"test".to_vec()),
        };

        let packet = factory.build_packet(&builder).unwrap();
        let assembler = PacketAssembler::new();
        
        let packet_bytes = assembler.assemble_packet(&packet).unwrap();
        let packet_size = assembler.get_packet_size(&packet);

        // Vérifier la taille
        assert_eq!(packet_bytes.len(), packet_size);
        assert!(packet_bytes.len() > 0);

        // UDP packets should be smaller than TCP
        assert!(packet_bytes.len() < 100);
    }

    ///////////////////////////////////////////
    ///      Individual Builder Tests        ///
    ///////////////////////////////////////////

    #[test]
    fn test_tcp_builder() {
        let tcp_builder = TcpBuilder::new(
            Ipv4Addr { octets: [192, 168, 1, 1] },
            Ipv4Addr { octets: [192, 168, 1, 2] },
        );

        let tcp_header = tcp_builder.build_tcp_header(
            8080,
            443,
            Some(b"test payload".to_vec()),
        ).unwrap();

        assert_eq!(tcp_header.src_port, 8080);
        assert_eq!(tcp_header.dst_port, 443);
        assert_eq!(tcp_header.payload, Some(b"test payload".to_vec()));
        assert!(tcp_header.checksum > 0); // Checksum calculé
    }

    #[test]
    fn test_udp_builder() {
        let udp_builder = UdpBuilder::new(
            Ipv4Addr { octets: [10, 0, 0, 1] },
            Ipv4Addr { octets: [10, 0, 0, 2] },
        );

        let udp_header = udp_builder.build_udp_header(
            53,
            53,
            Some(b"dns query".to_vec()),
        ).unwrap();

        assert_eq!(udp_header.src_port, 53);
        assert_eq!(udp_header.dst_port, 53);
        assert_eq!(udp_header.payload, Some(b"dns query".to_vec()));
        assert!(udp_header.checksum > 0); // Checksum calculé
    }

    #[test]
    fn test_ipv4_builder() {
        let ipv4_builder = Ipv4Builder::new(
            Ipv4Addr { octets: [192, 168, 1, 1] },
            Ipv4Addr { octets: [192, 168, 1, 2] },
            0x04, // Bitfield
        );

        let tcp_header = TcpHeader {
            src_port: 8080,
            dst_port: 443,
            sequence_number: 0,
            ack_nowledgment_number: 0,
            data_offset: 5,
            reserved: 0,
            flags: 0x02,
            window: 65535,
            checksum: 0,
            urgent_pointer: 0,
            options: None,
            payload: Some(b"test".to_vec()),
        };

        let l4_data = L4Data::Tcp(tcp_header);
        let ipv4_header = ipv4_builder.build_ipv4_header(&l4_data).unwrap();

        assert_eq!(ipv4_header.src_addr, [192, 168, 1, 1]);
        assert_eq!(ipv4_header.dst_addr, [192, 168, 1, 2]);
        assert_eq!(ipv4_header.protocol, 6); // TCP
        assert_eq!(ipv4_header.flags, 0); // Bitfield appliqué
        assert!(ipv4_header.header_checksum > 0); // Checksum calculé
    }

    #[test]
    fn test_ethernet_builder() {
        let ethernet_builder = EthernetBuilder::new();

        let ethernet_header = ethernet_builder.build_ethernet_header(
            [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        );

        assert_eq!(ethernet_header.src_mac, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(ethernet_header.dst_mac, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        assert_eq!(ethernet_header.ethertype, 0x0800); // IPv4
    }
}
