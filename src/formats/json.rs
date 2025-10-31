use crate::{
    structs::network_packet::NetworkPacket,
    structs::l4_protocol::L4Data,
    structs::json::{
        JsonPacket,
        JsonEthernet,
        JsonIpv4,
        JsonL4,
        JsonMetadata,
        JsonValue,
        JsonSerializer,
        JsonDeserializer
    },
    errors::errors::Result,
    utils::{
        payload_size::payload_len,
        formating_types::{
            format_mac,
            format_ip,
            format_bytes,
            get_timestamp_ms
        }
    },
    prelude::*,
};
use serde_json_core;

/// Serializer JSON pour les paquets reseau
impl JsonSerializer {
    
    /// constructeur JsonSerializer
    pub fn new() -> Self { 
        Self { 
            include_raw_data: true 
        }
    }

    /// constructeur JsonSerializer no data
    pub fn without_raw_data() -> Self { 
        Self { 
            include_raw_data: false 
        } 
    }

    // Serializer un seul paquet en JSON
    pub fn serialize_packet(&self, packet: &NetworkPacket) -> Result<StringNoStd> {
        let json_packet = self.convert_to_json_packet(packet)?;
        let mut buf = [0u8; 4096];
        let serialized_len = serde_json_core::ser::to_slice(&json_packet, &mut buf)?;
        Ok(StringNoStd::from_utf8_lossy(&buf[..serialized_len]).to_string())
    }

    // Convertir un paquet en un objet JSON
    fn convert_to_json_packet(&self, packet: &NetworkPacket) -> Result<JsonPacket> {
        let ethernet = JsonEthernet {
            src_mac: format_mac(
                &packet.ethernet.src_mac
            ),
            dst_mac: format_mac(
                &packet.ethernet.dst_mac
            ),
            ethertype: packet.ethernet.ethertype,
        };

        let ipv4 = JsonIpv4 {
            src_addr: format_ip(
                &packet.ipv4.src_addr
            ),
            dst_addr: format_ip(
                &packet.ipv4.dst_addr
            ),
            protocol: packet.ipv4.protocol,
            total_length: packet.ipv4.total_length,
            header_checksum: packet.ipv4.header_checksum,
            ttl: packet.ipv4.ttl,
            flags: packet.ipv4.flags,
            fragment_offset: packet.ipv4.fragment_offset,
        };

        let l4 = match &packet.l4_data {
            L4Data::Tcp(tcp) => {
                let mut additional_fields = BTreeMap::new();
                additional_fields.insert(
                    "sequence_number".to_string(), 
                    JsonValue::U64(
                        tcp.sequence_number as u64
                    )
                );
                additional_fields.insert(
                    "ack_number".to_string(), 
                    JsonValue::U64(
                        tcp.ack_nowledgment_number as u64
                    )
                );
                additional_fields.insert(
                    "flags".to_string(), 
                    JsonValue::U64(
                        tcp.flags as u64
                    )
                );
                additional_fields.insert(
                    "window".to_string(), 
                    JsonValue::U64(
                        tcp.window as u64
                    )
                );

                JsonL4 {
                    protocol_type: "TCP".to_string(),
                    src_port: tcp.src_port,
                    dst_port: tcp.dst_port,
                    payload_size: payload_len(&tcp.payload),
                    checksum: tcp.checksum,
                    additional_fields,
                }
            }
            L4Data::Udp(udp) => {
                let mut additional_fields = BTreeMap::new();
                additional_fields.insert("length".to_string(), JsonValue::U64(udp.length as u64));

                JsonL4 {
                    protocol_type: "UDP".to_string(),
                    src_port: udp.src_port,
                    dst_port: udp.dst_port,
                    payload_size: payload_len(&udp.payload),
                    checksum: udp.checksum,
                    additional_fields,
                }
            }
        };

        let raw_data = if self.include_raw_data {
            let packet_bytes = packet.assemble_packet()?;
            format_bytes(&packet_bytes)
        } else {
            StringNoStd::new()
        };

        let metadata = JsonMetadata {
            packet_size: packet.get_packet_size(),
            timestamp: get_timestamp_ms(),
            raw_data,
        };

        Ok(JsonPacket { ethernet, ipv4, l4, metadata })
    }

    // Serializer plusieurs paquets en JSON
    pub fn serialize_packets(&self, packets: &[NetworkPacket]) -> Result<StringNoStd> {
        let mut buf = [0u8; 8192];
        let mut json_packets = VecNoStd::new();
        for packet in packets {
            json_packets.push(
                self.convert_to_json_packet(packet)?
            );
        }
        let serialized_len = serde_json_core::ser::to_slice(&json_packets, &mut buf)?;
        Ok(StringNoStd::from_utf8_lossy(
            &buf[..serialized_len]
        ).to_string())
    }
}

/// Deserializer JSON pour les paquets reseau
impl JsonDeserializer {
    pub fn new() -> Self {
        JsonDeserializer
    }

    pub fn deserialize_packet(&self, json_str: &str) -> Result<JsonPacket> {
        let (packet, _rest) = serde_json_core::de::from_str(json_str)?;
        Ok(packet)
    }

    pub fn deserialize_packets(&self, json_str: &str) -> Result<VecNoStd<JsonPacket>> {
        let (packets, _rest) = serde_json_core::de::from_str(json_str)?;
        Ok(packets)
    }
}

