use serde::{Serialize, Deserialize};
use crate::prelude::*;

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
    pub include_raw_data: bool,
}

pub struct JsonDeserializer;
