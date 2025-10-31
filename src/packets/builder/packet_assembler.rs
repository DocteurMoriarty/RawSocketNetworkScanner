use alloc::vec::Vec;
use crate::{
    utils::payload_size::payload_len,
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

// Rend l'assembleur public
pub struct PacketAssembler;

// Assembleur de paquets réseau
impl PacketAssembler {

    // Constructor
    pub fn new() -> Self {
        Self
    }

    /// Assemble un paquet reseau complet
    pub fn assemble_packet(&self, packet: &NetworkPacket) -> Result<Vec<u8>> {
        let l4_data = match &packet.l4_data {
            L4Data::Tcp(tcp_header) => pack_tcp(tcp_header)?,
            L4Data::Udp(udp_header) => pack_udp(udp_header)?,
        };
        let ip_packet = pack_ipv4(&packet.ipv4, &l4_data)?;
        let ethernet_packet = pack_ethernet(&packet.ethernet, &ip_packet)?;
        Ok(ethernet_packet)
    }

    // Calcule la taille totale du paquet reseau
    pub fn get_packet_size(&self, packet: &NetworkPacket) -> usize {
        let l4_size = match &packet.l4_data {
            L4Data::Tcp(tcp) => 20 + payload_len(&tcp.payload),
            L4Data::Udp(udp) => 8 + payload_len(&udp.payload),
        };

        14 + 20 + l4_size
    }
}
