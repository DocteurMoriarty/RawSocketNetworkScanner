use alloc::vec::Vec;
use crate::{
    structs::{
        ipv4::Ipv4Addr,
        l4_protocol::{L4Data, L4Protocol},
        network_packet::NetworkPacket,
        packet_builder::PacketBuilder,
    },
    parsing::my_parser::parse_ipv4,
    errors::errors::{Result, ParseError},
};

// Fabrique de paquets réseau
pub struct PacketFactory {
    tcp_builder: super::tcp_builder::TcpBuilder,
    udp_builder: super::udp_builder::UdpBuilder,
    ipv4_builder: super::ipv4_builder::Ipv4Builder,
    ethernet_builder: super::ethernet_builder::EthernetBuilder,
}

// Implementation de PacketFactory
impl PacketFactory {

    // Constructor
    pub fn new(src_ip: Ipv4Addr, dst_ip: Ipv4Addr, ip_bitfield: u8) -> Self {
        Self {
            tcp_builder: super::tcp_builder::TcpBuilder::new(src_ip, dst_ip),
            udp_builder: super::udp_builder::UdpBuilder::new(src_ip, dst_ip),
            ipv4_builder: super::ipv4_builder::Ipv4Builder::new(src_ip, dst_ip, ip_bitfield),
            ethernet_builder: super::ethernet_builder::EthernetBuilder::new(),
        }
    }

    /// Construit un paquet reseau complet à partir des information du PacketBuilder
    pub fn build_packet(&self, builder: &PacketBuilder) -> Result<NetworkPacket> {
        let l4_data = match builder.protocol {
            L4Protocol::Tcp => {
                let tcp_header = self.tcp_builder.build_tcp_header(
                    builder.src_port,
                    builder.dst_port,
                    builder.payload.clone(),
                )?;
                L4Data::Tcp(tcp_header)
            }
            L4Protocol::Udp => {
                let udp_header = self.udp_builder.build_udp_header(
                    builder.src_port,
                    builder.dst_port,
                    builder.payload.clone(),
                )?;
                L4Data::Udp(udp_header)
            }
        };

        let ipv4_header = self.ipv4_builder.build_ipv4_header(&l4_data)?;
        let ethernet_header = self.ethernet_builder.build_ethernet_header(
            builder.src_mac,
            builder.dst_mac,
        );

        Ok(NetworkPacket {
            ethernet: ethernet_header,
            ipv4: ipv4_header,
            l4_data,
        })
    }

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
    ) -> Result<(Self, PacketBuilder)> {
        let src_ip = parse_ipv4(
            src_ip.ok_or(
                ParseError::MissingRequiredField("src_ip")
            )?
        )?;

        let dst_ip = parse_ipv4(
            dst_ip.ok_or(
                ParseError::MissingRequiredField("dst_ip")
            )?
        )?;

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

        let packet_builder = PacketBuilder {
            src_ip,
            dst_ip,
            src_mac,
            dst_mac,
            src_port,
            dst_port,
            protocol,
            ip_bitfield,
            payload,
        };

        let factory = Self::new(src_ip, dst_ip, ip_bitfield);

        Ok(
            (
                factory, 
                packet_builder
            )
        )
    }
}
