use alloc::vec::Vec;
use crate::{
    structs::{
        udp::UdpHeader,
        ipv4::Ipv4Addr,
    },
    packets::l4::udp::pack_udp,
    utils::{
        checksum::internet_checksum,
        payload_size::payload_len
    },
    errors::errors::Result,
};

// Constructeur de paquets UDP
pub struct UdpBuilder {
    src_ip: Ipv4Addr,
    dst_ip: Ipv4Addr,
}

// Implementation de UdpBuilder
impl UdpBuilder {

    // Constructor
    pub fn new(
        src_ip: Ipv4Addr, 
        dst_ip: Ipv4Addr
    ) -> Self {
        Self { 
            src_ip, 
            dst_ip 
        }
    }

    /// Construit l'header UDP
    pub fn build_udp_header(
        &self,
        src_port: u16,
        dst_port: u16,
        payload: Option<Vec<u8>>,
    ) -> 
    Result<
        UdpHeader
    > {
        let udp_length = 8 + payload_len(&payload);

        let mut udp_header = UdpHeader {
            src_port,
            dst_port,
            length: udp_length as u16,
            checksum: 0,
            payload,
        };

        udp_header.checksum = self.calculate_udp_checksum(&udp_header)?;

        Ok(udp_header)
    }

    // Calcule le checksum UDP
    fn calculate_udp_checksum(&self, udp_header: &UdpHeader) -> Result<u16> {
        let mut pseudo_header = Vec::new();
        pseudo_header.extend_from_slice(&self.src_ip.octets);
        pseudo_header.extend_from_slice(&self.dst_ip.octets);
        pseudo_header.push(0);
        pseudo_header.push(17);
        pseudo_header.extend_from_slice(&udp_header.length.to_be_bytes());
        let udp_datagram = pack_udp(udp_header)?;
        let mut checksum_data = pseudo_header;
        checksum_data.extend_from_slice(&udp_datagram);
        Ok(
            internet_checksum(&checksum_data)
        )
    }
}
