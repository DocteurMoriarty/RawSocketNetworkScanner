use crate::{
    structs::network_packet::NetworkPacket,
    errors::errors::Result,
    prelude::*,
};
use crate::structs::{
    formats::FormatType,
    pcap::{PcapReader, PcapWriter},
    json::{JsonSerializer, JsonDeserializer}
};

pub struct FormatFactory;

impl FormatFactory {
    pub fn new() -> Self {
        Self
    }

    pub fn create_writer(&self, format_type: FormatType) -> Box<dyn FormatWriter> {
        match format_type {
            FormatType::Pcap => Box::new(PcapWriter::new()),
            FormatType::Json => Box::new(JsonSerializer::new()),
        }
    }

    pub fn create_reader(&self, format_type: FormatType, data: VecNoStd<u8>) -> Box<dyn FormatReader> {
        match format_type {
            FormatType::Pcap => Box::new(PcapReader::new(data)),
            FormatType::Json => Box::new(JsonDeserializer::new()),
        }
    }

    pub fn write_packet(&self, packet: &NetworkPacket, format_type: FormatType) -> Result<VecNoStd<u8>> {
        match format_type {
            FormatType::Pcap => {
                let mut writer = PcapWriter::new();
                writer.write_global_header()?;
                writer.write_packet(packet)?;
                Ok(writer.into_data())
            }
            FormatType::Json => {
                let serializer = JsonSerializer::new();
                Ok(serializer.serialize_packet(packet)?.into_bytes())
            }
        }
    }

    pub fn write_packets(&self, packets: &[NetworkPacket], format_type: FormatType) -> Result<VecNoStd<u8>> {
        match format_type {
            FormatType::Pcap => {
                let mut writer = PcapWriter::new();
                writer.write_global_header()?;
                for packet in packets {
                    writer.write_packet(packet)?;
                }
                Ok(writer.into_data())
            }
            FormatType::Json => {
                let serializer = JsonSerializer::new();
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

impl FormatWriter for PcapWriter {
    fn write_packet(&mut self, packet: &NetworkPacket) -> Result<()> {
        self.write_packet(packet)
    }

    fn get_data(&self) -> &[u8] {
        self.get_data()
    }

    fn into_data(self: Box<Self>) -> VecNoStd<u8> {
        (*self).into_data()
    }
}

impl FormatWriter for JsonSerializer {
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

impl FormatReader for PcapReader {
    fn read_next_packet(&mut self) -> Result<Option<VecNoStd<u8>>> {
        self.read_next_packet()
    }

    fn has_more_packets(&self) -> bool {
        self.has_more_packets()
    }
}

impl FormatReader for JsonDeserializer {
    fn read_next_packet(&mut self) -> Result<Option<VecNoStd<u8>>> {
        Ok(None)
    }

    fn has_more_packets(&self) -> bool {
        false
    }
}
