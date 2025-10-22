use serde_json_core::ser::to_string;
use serde_json_core::heapless::String as HString;
use crate::utils::format_mac::mac_to_string;
use crate::prelude::*;

use crate::{
    structs::network_packet::NetworkPacket,
    structs::l4_protocol::L4Data,
    errors::errors::Result,
};
use serde::{Serialize, Deserialize};
use serde_json_core;

// Enum pour remplacer serde_json::Value
#[derive(Serialize, Deserialize, Debug)]
pub enum JsonValue {
    U64(u64),
    Bool(bool),
    String(StringNoStd),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonPacket {
    pub ethernet: JsonEthernet,
    pub ipv4: JsonIpv4,
    pub l4: JsonL4,
    pub metadata: JsonMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEthernet {
    pub src_mac: StringNoStd,
    pub dst_mac: StringNoStd,
    pub ethertype: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonIpv4 {
    pub src_addr: StringNoStd,
    pub dst_addr: StringNoStd,
    pub protocol: u8,
    pub total_length: u16,
    pub header_checksum: u16,
    pub ttl: u8,
    pub flags: u8,
    pub fragment_offset: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonL4 {
    pub protocol_type: StringNoStd,
    pub src_port: u16,
    pub dst_port: u16,
    pub payload_size: usize,
    pub checksum: u16,
    pub additional_fields: BTreeMap<StringNoStd, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonMetadata {
    pub packet_size: usize,
    pub timestamp: u64, 
    pub raw_data: StringNoStd,
}

pub struct JsonSerializer {
    include_raw_data: bool,
}

impl JsonSerializer {
    pub fn new() -> Self { Self { include_raw_data: true } }
    pub fn without_raw_data() -> Self { Self { include_raw_data: false } }

    pub fn serialize_packet(&self, packet: &NetworkPacket) -> Result<StringNoStd> {
        let json_packet = self.convert_to_json_packet(packet)?;
        let s = serde_json_core::ser::to_string(&json_packet)?;
        Ok(s.as_str().to_string())
    }



    pub fn serialize_packet(&self, packet: &NetworkPacket) -> Result<StringNoStd> {
        let json_packet = self.convert_to_json_packet(packet)?;
        let s: HString<4096> = to_string(&json_packet)?;        
        Ok(s.as_str().to_string())
    }

    fn convert_to_json_packet(&self, packet: &NetworkPacket) -> Result<JsonPacket> {
        let ethernet = JsonEthernet {
            src_mac: mac_to_string!("{:02X?}", packet.ethernet.src_mac),
            dst_mac: mac_to_string!("{:02X?}", packet.ethernet.dst_mac),
            ethertype: packet.ethernet.ethertype,
        };

        let ipv4 = JsonIpv4 {
            src_addr: mac_to_string!("{}.{}.{}.{}", packet.ipv4.src_addr[0], packet.ipv4.src_addr[1], packet.ipv4.src_addr[2], packet.ipv4.src_addr[3]),
            dst_addr: mac_to_string!("{}.{}.{}.{}", packet.ipv4.dst_addr[0], packet.ipv4.dst_addr[1], packet.ipv4.dst_addr[2], packet.ipv4.dst_addr[3]),
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
                additional_fields.insert("sequence_number".to_string(), JsonValue::U64(tcp.sequence_number as u64));
                additional_fields.insert("ack_number".to_string(), JsonValue::U64(tcp.ack_nowledgment_number as u64));
                additional_fields.insert("flags".to_string(), JsonValue::U64(tcp.flags as u64));
                additional_fields.insert("window".to_string(), JsonValue::U64(tcp.window as u64));

                JsonL4 {
                    protocol_type: "TCP".to_string(),
                    src_port: tcp.src_port,
                    dst_port: tcp.dst_port,
                    payload_size: tcp.payload.as_ref().map(|p| p.len()).unwrap_or(0),
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
                    payload_size: udp.payload.as_ref().map(|p| p.len()).unwrap_or(0),
                    checksum: udp.checksum,
                    additional_fields,
                }
            }
        };

        let raw_data = if self.include_raw_data {
            let packet_bytes = packet.assemble_packet()?;
            mac_to_string!("{:02X?}", packet_bytes)
        } else {
            StringNoStd::new()
        };

        let metadata = JsonMetadata {
            packet_size: packet.get_packet_size(),
            timestamp: packet.get_timestamp_ms(),
            raw_data,
        };

        Ok(JsonPacket { ethernet, ipv4, l4, metadata })
    }
}

pub struct JsonDeserializer;

impl JsonDeserializer {
    pub fn new() -> Self { Self }

    pub fn deserialize_packet(&self, json_str: &str) -> Result<JsonPacket> {
        let (_rest, pkt) = serde_json_core::de::from_str(json_str)?;
        Ok(pkt)
    }

    pub fn deserialize_packets(&self, json_str: &str) -> Result<Vec<JsonPacket>> {
        let (_rest, pkts) = serde_json_core::de::from_str(json_str)?;
        Ok(pkts)
    }
}
