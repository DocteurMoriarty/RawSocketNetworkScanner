use alloc::vec::Vec;
use crate::{
    packets::{
        ethernet::pack_ethernet,
        ip::pack_ipv4,
        l4::{tcp::pack_tcp, udp::pack_udp},
    },
    structs::{
        network_packet::NetworkPacket,
        l4_protocol::L4Data,
    },
    errors::errors::Result,
};

pub struct PacketAssembler;

impl PacketAssembler {
    pub fn new() -> Self {
        Self
    }

    pub fn assemble_packet(&self, packet: &NetworkPacket) -> Result<Vec<u8>> {
        let l4_data = match &packet.l4_data {
            L4Data::Tcp(tcp_header) => pack_tcp(tcp_header)?,
            L4Data::Udp(udp_header) => pack_udp(udp_header)?,
        };
        let ip_packet = pack_ipv4(&packet.ipv4, &l4_data)?;
        let ethernet_packet = pack_ethernet(&packet.ethernet, &ip_packet)?;
        Ok(ethernet_packet)
    }

    pub fn get_packet_size(&self, packet: &NetworkPacket) -> usize {
        let l4_size = match &packet.l4_data {
            L4Data::Tcp(tcp) => 20 + tcp.payload.as_ref().map(|p| p.len()).unwrap_or(0),
            L4Data::Udp(udp) => 8 + udp.payload.as_ref().map(|p| p.len()).unwrap_or(0),
        };

        14 + 20 + l4_size
    }
}
