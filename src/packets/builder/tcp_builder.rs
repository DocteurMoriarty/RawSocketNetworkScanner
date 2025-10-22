use alloc::vec::Vec;
use crate::{
    structs::{
        tcp::TcpHeader,
        ipv4::Ipv4Addr,
    },
    packets::l4::tcp::pack_tcp,
    utils::checksum::internet_checksum,
    errors::errors::Result,
};

pub struct TcpBuilder {
    src_ip: Ipv4Addr,
    dst_ip: Ipv4Addr,
}

impl TcpBuilder {
    pub fn new(src_ip: Ipv4Addr, dst_ip: Ipv4Addr) -> Self {
        Self { src_ip, dst_ip }
    }

    pub fn build_tcp_header(
        &self,
        src_port: u16,
        dst_port: u16,
        payload: Option<Vec<u8>>,
    ) -> Result<TcpHeader> {
        let mut tcp_header = TcpHeader {
            src_port,
            dst_port,
            sequence_number: 0,
            ack_nowledgment_number: 0,
            data_offset: 5,
            reserved: 0,
            flags: 0x02,
            window: 65535,
            checksum: 0,
            urgent_pointer: 0,
            options: None,
            payload,
        };
        tcp_header.checksum = self.calculate_tcp_checksum(&tcp_header)?;

        Ok(tcp_header)
    }

    fn calculate_tcp_checksum(&self, tcp_header: &TcpHeader) -> Result<u16> {
        let mut pseudo_header = Vec::new();
        pseudo_header.extend_from_slice(&self.src_ip.octets);
        pseudo_header.extend_from_slice(&self.dst_ip.octets);
        pseudo_header.push(0);
        pseudo_header.push(6);
        let tcp_length = 20 + tcp_header.payload.as_ref().map(|p| p.len()).unwrap_or(0);
        pseudo_header.extend_from_slice(&(tcp_length as u16).to_be_bytes());

        let tcp_segment = pack_tcp(tcp_header)?;

        let mut checksum_data = pseudo_header;
        checksum_data.extend_from_slice(&tcp_segment);

        Ok(internet_checksum(&checksum_data))
    }
}
