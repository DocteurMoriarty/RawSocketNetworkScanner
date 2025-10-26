use crate::{
    structs::network_packet::NetworkPacket,
    structs::pcap::{PcapWriter, PcapReader},
    errors::errors::Result,
    prelude::*,
};

impl PcapWriter {
    pub fn new() -> Self {
        Self {
            buffer: VecNoStd::new(),
        }
    }

    pub fn write_global_header(&mut self) -> Result<()> {
        let global_header = [
            0xD4, 0xC3, 0xB2, 0xA1, 
            0x02, 0x00, 0x04, 0x00, 
            0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00, 
            0xFF, 0xFF, 0x00, 0x00, 
            0x01, 0x00, 0x00, 0x00,
        ];
        self.buffer.extend_from_slice(&global_header);
        Ok(())
    }

    pub fn write_packet(&mut self, packet: &NetworkPacket) -> Result<()> {
        let packet_data = packet.assemble_packet()?;
        let timestamp = get_timestamp();

        let packet_header = [
            (timestamp & 0xFFFFFFFF) as u32,
            ((timestamp >> 32) & 0xFFFFFFFF) as u32,
            packet_data.len() as u32,
            packet_data.len() as u32,
        ];

        for &value in &packet_header {
            self.buffer.extend_from_slice(&value.to_le_bytes());
        }
        self.buffer.extend_from_slice(&packet_data);

        Ok(())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.buffer
    }

    pub fn into_data(self) -> VecNoStd<u8> {
        self.buffer
    }
}

impl PcapReader {
    pub fn new(data: VecNoStd<u8>) -> Self {
        Self {
            data,
            position: 0,
        }
    }

    pub fn read_global_header(&mut self) -> Result<()> {
        if self.data.len() < 24 {
            return Err(crate::errors::errors::ParseError::InvalidFormat("PCAP header too short").into());
        }

        let magic = u32::from_le_bytes([
            self.data[0], self.data[1], self.data[2], self.data[3]
        ]);

        if magic != 0xA1B2C3D4 {
            return Err(crate::errors::errors::ParseError::InvalidFormat("Invalid PCAP magic number").into());
        }

        self.position = 24;
        Ok(())
    }

    pub fn read_next_packet(&mut self) -> Result<Option<VecNoStd<u8>>> {
        if self.position + 16 > self.data.len() {
            return Ok(None);
        }

        let _timestamp_sec = u32::from_le_bytes([
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ]);

        let _timestamp_usec = u32::from_le_bytes([
            self.data[self.position + 4],
            self.data[self.position + 5],
            self.data[self.position + 6],
            self.data[self.position + 7],
        ]);

        let caplen = u32::from_le_bytes([
            self.data[self.position + 8],
            self.data[self.position + 9],
            self.data[self.position + 10],
            self.data[self.position + 11],
        ]);

        let _len = u32::from_le_bytes([
            self.data[self.position + 12],
            self.data[self.position + 13],
            self.data[self.position + 14],
            self.data[self.position + 15],
        ]);

        self.position += 16;

        if self.position + caplen as usize > self.data.len() {
            return Err(crate::errors::errors::ParseError::InvalidFormat("PCAP packet data truncated").into());
        }

        let packet_data = self.data[self.position..self.position + caplen as usize].to_vec();
        self.position += caplen as usize;

        Ok(Some(packet_data))
    }

    pub fn has_more_packets(&self) -> bool {
        self.position < self.data.len()
    }
}

fn get_timestamp() -> u64 {
    // En mode no_std, on utilise un timestamp simple
    // Dans un vrai environnement, on utiliserait un timer hardware
    0
}
