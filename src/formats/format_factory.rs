use crate::{
    structs::network_packet::NetworkPacket,
    errors::errors::Result,
};

use crate::prelude::*;

pub enum FormatType {
    Pcap,
    Json,
}

pub struct FormatFactory;

impl FormatFactory {
    pub fn new() -> Self {
        Self
    }

    pub fn create_writer(&self, format_type: FormatType) -> Box<dyn FormatWriter> {
        match format_type {
            FormatType::Pcap => Box::new(super::pcap::PcapWriter::new()),
            FormatType::Json => Box::new(super::json::JsonSerializer::new()),
        }
    }

    pub fn create_reader(&self, format_type: FormatType, data: VecNoStd<u8>) -> Box<dyn FormatReader> {
        match format_type {
            FormatType::Pcap => Box::new(super::pcap::PcapReader::new(data)),
            FormatType::Json => Box::new(super::json::JsonDeserializer::new()),
        }
    }

    pub fn write_packet(&self, packet: &NetworkPacket, format_type: FormatType) -> Result<VecNoStd<u8>> {
        match format_type {
            FormatType::Pcap => {
                let mut writer = super::pcap::PcapWriter::new();
                writer.write_global_header()?;
                writer.write_packet(packet)?;
                Ok(writer.into_data())
            }
            FormatType::Json => {
                let serializer = super::json::JsonSerializer::new();
                Ok(serializer.serialize_packet(packet)?.into_bytes())
            }
        }
    }

    pub fn write_packets(&self, packets: &[NetworkPacket], format_type: FormatType) -> Result<VecNoStd<u8>> {
        match format_type {
            FormatType::Pcap => {
                let mut writer = super::pcap::PcapWriter::new();
                writer.write_global_header()?;
                for packet in packets {
                    writer.write_packet(packet)?;
                }
                Ok(writer.into_data())
            }
            FormatType::Json => {
                let serializer = super::json::JsonSerializer::new();
                Ok(serializer.serialize_packets(packets)?.into_bytes())
            }
        }
    }
}

pub trait FormatWriter {
    fn write_packet(&mut self, packet: &NetworkPacket) -> Result<()>;
    fn get_data(&self) -> &[u8];
    fn into_data(self: Box<Self>) -> VecNoStd<u8>;
}

pub trait FormatReader {
    fn read_next_packet(&mut self) -> Result<Option<VecNoStd<u8>>>;
    fn has_more_packets(&self) -> bool;
}

impl FormatWriter for super::pcap::PcapWriter {
    fn write_packet(&mut self, packet: &NetworkPacket) -> Result<()> {
        super::pcap::PcapWriter::write_packet(self, packet)
    }

    fn get_data(&self) -> &[u8] {
        super::pcap::PcapWriter::get_data(self)
    }

    fn into_data(self: Box<Self>) -> VecNoStd<u8> {
        super::pcap::PcapWriter::into_data(*self)
    }
}

impl FormatWriter for super::json::JsonSerializer {
    fn write_packet(&mut self, packet: &NetworkPacket) -> Result<()> {
        let _json = self.serialize_packet(packet)?;
        Ok(())
    }

    fn get_data(&self) -> &[u8] {
        &[]
    }

    fn into_data(self: Box<Self>) -> VecNoStd<u8> {
        VecNoStd::new()
    }
}

impl FormatReader for super::pcap::PcapReader {
    fn read_next_packet(&mut self) -> Result<Option<VecNoStd<u8>>> {
        super::pcap::PcapReader::read_next_packet(self)
    }

    fn has_more_packets(&self) -> bool {
        super::pcap::PcapReader::has_more_packets(self)
    }
}

impl FormatReader for super::json::JsonDeserializer {
    fn read_next_packet(&mut self) -> Result<Option<VecNoStd<u8>>> {
        Ok(None)
    }

    fn has_more_packets(&self) -> bool {
        false
    }
}
