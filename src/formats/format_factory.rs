

use crate::{
    structs::network_packet::NetworkPacket,
    errors::errors::Result,
    prelude::*,
};
use crate::structs::{
    formats::FormatType,
    pcap::{
        PcapReader,
        PcapWriter
    },
    json::{
        JsonSerializer,
        JsonDeserializer
    }
};

pub struct FormatFactory;

impl FormatFactory {
    pub fn new() -> Self {
        Self
    }

    // Creer un constructeur pour le format specifie (Pcap ou Json)
    pub fn create_writer(&self, format_type: FormatType) -> Box<dyn FormatWriter> {
        match format_type {
            FormatType::Pcap => Box::new(PcapWriter::new()),
            FormatType::Json => Box::new(JsonSerializer::new()),
        }
    }

    // Creer un lecteur pour le format specifie (Pcap ou Json)
    pub fn create_reader(&self, format_type: FormatType, data: VecNoStd<u8>) -> Box<dyn FormatReader> {
        match format_type {
            FormatType::Pcap => Box::new(PcapReader::new(data)),
            FormatType::Json => Box::new(JsonDeserializer::new()),
        }
    }

    // Ecrire un seul paquet dans le format specifie (Pcap ou Json)
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

    // Ecrire plusieurs paquets dans le format specifie (Pcap ou Json)
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

// Trait pour l'ecriture des formats de paquets
pub trait FormatWriter {
    fn write_packet(&mut self, packet: &NetworkPacket) -> Result<()>;
    fn get_data(&self) -> &[u8];
    fn into_data(self: Box<Self>) -> VecNoStd<u8>;
}

// Trait pour la lecture des formats de paquets
pub trait FormatReader {
    fn read_next_packet(&mut self) -> Result<Option<VecNoStd<u8>>>;
    fn has_more_packets(&self) -> bool;
}

// Implementation de FormatWriter pour PcapWriter
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

// Implementation de FormatWriter pour JsonSerializer
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

// Implementation de FormatReader pour PcapReader
impl FormatReader for PcapReader {
    fn read_next_packet(&mut self) -> Result<Option<VecNoStd<u8>>> {
        self.read_next_packet()
    }

    fn has_more_packets(&self) -> bool {
        self.has_more_packets()
    }
}

// Implementation de FormatReader pour JsonDeserializer
impl FormatReader for JsonDeserializer {
    fn read_next_packet(&mut self) -> Result<Option<VecNoStd<u8>>> {
        Ok(None)
    }

    fn has_more_packets(&self) -> bool {
        false
    }
}
