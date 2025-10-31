#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        structs::{
            network_packet::NetworkPacket,
            ethernet::EthernetHeader,
            ip::Ipv4Header,
            tcp::TcpHeader,
            l4_protocol::L4Data,
        },
    };

    use projet_rsns_morissetlarresacha::structs::socket::RawSocketSender;
    use projet_rsns_morissetlarresacha::sender::raw_socket::get_interface_index;

    ///////////////////////////////////////////
    ///      Raw Socket Tests               ///
    ///////////////////////////////////////////

    fn create_test_packet() -> NetworkPacket {
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

    #[test]
    fn test_raw_socket_creation() {
        let result = RawSocketSender::new();
        if let Ok(_sender) = result {
        }
    }

    #[test]
    fn test_raw_socket_timeout() {
        if let Ok(sender) = RawSocketSender::new() {
            let result = sender.set_write_timeout(Some(1000)); // 1 seconde
            assert!(result.is_ok());
            
            let result = sender.set_write_timeout(None);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_interface_index_detection() {
        let interfaces = ["lo", "eth0", "enp0s3", "wlan0"];
        
        for iface in &interfaces {
            let result = get_interface_index(iface);
            if let Ok(index) = result {
                assert!(index > 0);
                println!("Interface {} a l'index {}", iface, index);
            }
        }
    }

    #[test]
    fn test_raw_socket_send_dry_run() {
        let packet = create_test_packet();
        let packet_bytes = packet.assemble_packet().unwrap();
        
        if let Ok(sender) = RawSocketSender::new() {
            if let Ok(if_index) = get_interface_index("lo") {
                let result = sender.send(if_index, packet.ethernet.dst_mac, &packet_bytes);
                
                match result {
                    Ok(bytes_sent) => {
                        assert!(bytes_sent > 0);
                        println!("Paquet envoyé: {} octets", bytes_sent);
                    }
                    Err(e) => {
                        println!("Erreur d'envoi (normal sans privilèges): {}", e);
                    }
                }
            }
        }
    }

    #[test]
    fn test_packet_assembly_for_sending() {
        let packet = create_test_packet();
        let packet_bytes = packet.assemble_packet().unwrap();
        
        assert!(packet_bytes.len() > 0);
        assert!(packet_bytes.len() < 1000);
        
        assert_eq!(packet_bytes[0..6], packet.ethernet.dst_mac);
        assert_eq!(packet_bytes[6..12], packet.ethernet.src_mac);
        assert_eq!(packet_bytes[12..14], [0x08, 0x00]);
    }

    #[test]
    fn test_timeout_values() {
        if let Ok(sender) = RawSocketSender::new() {
            let timeouts = [100, 500, 1000, 5000];
            
            for timeout in &timeouts {
                let result = sender.set_write_timeout(Some(*timeout));
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn test_interface_validation() {
        let invalid_interfaces = ["nonexistent", "invalid_interface", ""];
        
        for iface in &invalid_interfaces {
            let result = get_interface_index(iface);
            assert!(result.is_err());
        }
    }

    ///////////////////////////////////////////
    ///      Integration Tests              ///
    ///////////////////////////////////////////

    #[test]
    fn test_full_packet_workflow() {
        let packet = create_test_packet();
        let packet_bytes = packet.assemble_packet().unwrap();
        
        assert!(packet_bytes.len() > 0);
        
        if let Ok(sender) = RawSocketSender::new() {
            if let Ok(if_index) = get_interface_index("lo") {
                sender.set_write_timeout(Some(1000)).unwrap();
                
                let result = sender.send(if_index, packet.ethernet.dst_mac, &packet_bytes);
                
                match result {
                    Ok(bytes_sent) => {
                        assert_eq!(bytes_sent, packet_bytes.len());
                    }
                    Err(_) => {
                    }
                }
            }
        }
    }

    #[test]
    fn test_multiple_packets() {
        let packets = vec![
            create_test_packet(),
            create_test_packet(),
        ];
        
        if let Ok(sender) = RawSocketSender::new() {
            if let Ok(if_index) = get_interface_index("lo") {
                sender.set_write_timeout(Some(1000)).unwrap();
                
                for packet in &packets {
                    let packet_bytes = packet.assemble_packet().unwrap();
                    let result = sender.send(if_index, packet.ethernet.dst_mac, &packet_bytes);
                    
                    match result {
                        Ok(bytes_sent) => {
                            assert_eq!(bytes_sent, packet_bytes.len());
                        }
                        Err(_) => {
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_error_handling() {
        let invalid_data = vec![0u8; 0];
        
        if let Ok(sender) = RawSocketSender::new() {
            if let Ok(if_index) = get_interface_index("lo") {
                let result = sender.send(if_index, [0xFF; 6], &invalid_data);
                
                assert!(result.is_err());
            }
        }
    }
}
