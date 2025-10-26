use alloc::vec::Vec;
use crate::{
    structs::{
        l4_protocol::L4Protocol,
        packet_builder::PacketBuilder,
    },
    parsing::my_parser::parse_ipv4,
    errors::errors::Result,
};



impl PacketBuilder {
    pub fn build_packet(&self) -> Result<crate::structs::network_packet::NetworkPacket> {
        let factory = super::packet_factory::PacketFactory::new(
            self.src_ip,
            self.dst_ip,
            self.ip_bitfield,
        );
        factory.build_packet(self)
    }
}


impl PacketBuilder {
    pub fn from_cli_args(
        src_ip: Option<&str>,
        dst_ip: Option<&str>,
        src_mac: Option<[u8; 6]>,
        dst_mac: Option<[u8; 6]>,
        src_port: Option<u16>,
        dst_port: Option<u16>,
        l4_protocol: Option<&str>,
        ip_bitfield: Option<u8>,
        payload: Option<Vec<u8>>,
    ) -> Result<Self> {
        let src_ip = if let Some(ip) = src_ip {
            parse_ipv4(ip)?
        } else {
            parse_ipv4("192.168.1.100")? // IP par défaut
        };

        let dst_ip = if let Some(ip) = dst_ip {
            parse_ipv4(ip)?
        } else {
            parse_ipv4("192.168.1.1")? // IP par défaut
        };

        let src_mac = src_mac.unwrap_or([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let dst_mac = dst_mac.unwrap_or([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

        let src_port = src_port.unwrap_or(12345);
        let dst_port = dst_port.unwrap_or(80);
        let protocol = match l4_protocol.as_deref() {
            Some("tcp") => L4Protocol::Tcp,
            Some("udp") => L4Protocol::Udp,
            _ => L4Protocol::Tcp,
        };
        let ip_bitfield = ip_bitfield.unwrap_or(0x00);
        Ok(PacketBuilder {
            src_ip,
            dst_ip,
            src_mac,
            dst_mac,
            src_port,
            dst_port,
            protocol,
            ip_bitfield,
            payload,
        })
    }
}
