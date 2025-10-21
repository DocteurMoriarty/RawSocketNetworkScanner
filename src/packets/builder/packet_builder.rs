use crate::{
    structs::{
        ethernet::EthernetHeader,
        ip::Ipv4Header,
        tcp::TcpHeader,
        udp::UdpHeader,
        ipv4::Ipv4Addr,
        l4_protocol::{
            L4Data, 
            L4Protocol
        },
        network_packet::NetworkPacket,
        packet_builder::PacketBuilder,
    },
    packets::{
        ethernet::pack_ethernet,
        ip::pack_ipv4,
        l4::{tcp::pack_tcp, udp::pack_udp},
    },
    utils::checksum::internet_checksum,
    parsing::my_parser::parse_ipv4,
    errors::errors::{
        Result,
        ParseError
    },
};



impl PacketBuilder {
    pub fn build_packet(&self) -> Result<NetworkPacket> {
        let l4_data = match self.protocol {
            L4Protocol::Tcp => {
                let tcp_header = self.build_tcp_header()?;
                L4Data::Tcp(tcp_header)
            }
            L4Protocol::Udp => {
                let udp_header = self.build_udp_header()?;
                L4Data::Udp(udp_header)
            }
        };

        let ipv4_header = self.build_ipv4_header(&l4_data)?;

        let ethernet_header = self.build_ethernet_header();

        Ok(NetworkPacket {
            ethernet: ethernet_header,
            ipv4: ipv4_header,
            l4_data,
        })
    }

    fn build_tcp_header(&self) -> Result<TcpHeader> {
        let payload = self.payload.clone();
        let payload_len = payload.as_ref().map(|p| p.len()).unwrap_or(0);

        let mut tcp_header = TcpHeader {
            src_port: self.src_port,
            dst_port: self.dst_port,
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

    fn build_udp_header(&self) -> Result<UdpHeader> {
        let payload = self.payload.clone();
        let payload_len = payload.as_ref().map(|p| p.len()).unwrap_or(0);
        
        let udp_length = 8 + payload_len;

        let mut udp_header = UdpHeader {
            src_port: self.src_port,
            dst_port: self.dst_port,
            length: udp_length as u16,
            checksum: 0,
            payload,
        };

        udp_header.checksum = self.calculate_udp_checksum(&udp_header)?;

        Ok(udp_header)
    }

    fn build_ipv4_header(&self, l4_data: &L4Data) -> Result<Ipv4Header> {
        let l4_length = match l4_data {
            L4Data::Tcp(tcp) => {
                let payload_len = tcp.payload.as_ref().map(|p| p.len()).unwrap_or(0);
                20 + payload_len
            }
            L4Data::Udp(udp) => {
                let payload_len = udp.payload.as_ref().map(|p| p.len()).unwrap_or(0);
                8 + payload_len
            }
        };

        let total_length = 20 + l4_length;

        let mut ipv4_header = Ipv4Header {
            version: 4,
            ihl: 5,
            dscp: 0,
            total_length: total_length as u16,
            identification: 0,
            flags: (self.ip_bitfield >> 5) & 0x07,
            fragment_offset: ((self.ip_bitfield & 0x1F) as u16) << 8,
            ttl: 64,
            protocol: match self.protocol {
                L4Protocol::Tcp => 6,
                L4Protocol::Udp => 17,
            },
            header_checksum: 0,
            src_addr: self.src_ip.octets,
            dst_addr: self.dst_ip.octets,
            options: None,
        };

        ipv4_header.header_checksum = self.calculate_ipv4_checksum(&ipv4_header)?;

        Ok(ipv4_header)
    }

    fn build_ethernet_header(&self) -> EthernetHeader {
        EthernetHeader {
            dst_mac: self.dst_mac,
            src_mac: self.src_mac,
            ethertype: 0x0800,
        }
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
        Ok(internet_checksum(&checksum_data))
    }

    fn calculate_ipv4_checksum(&self, ipv4_header: &Ipv4Header) -> Result<u16> {
        let mut temp_header = ipv4_header.clone();
        temp_header.header_checksum = 0;
        let ip_header_bytes = pack_ipv4(&temp_header, &[])?;
        let header_for_checksum = &ip_header_bytes[..20];        
        Ok(internet_checksum(header_for_checksum))
    }
}

impl NetworkPacket {
    pub fn assemble_packet(&self) -> Result<Vec<u8>> {
        let l4_data = match &self.l4_data {
            L4Data::Tcp(tcp_header) => pack_tcp(tcp_header)?,
            L4Data::Udp(udp_header) => pack_udp(udp_header)?,
        };
        let ip_packet = pack_ipv4(&self.ipv4, &l4_data)?;
        let ethernet_packet = pack_ethernet(&self.ethernet, &ip_packet)?;
        Ok(ethernet_packet)
    }

    pub fn get_packet_size(&self) -> usize {
        let l4_size = match &self.l4_data {
            L4Data::Tcp(tcp) => 20 + tcp.payload.as_ref().map(|p| p.len()).unwrap_or(0),
            L4Data::Udp(udp) => 8 + udp.payload.as_ref().map(|p| p.len()).unwrap_or(0),
        };
        
        14 + 20 + l4_size
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

        let src_ip = parse_ipv4(
            src_ip.ok_or(ParseError::MissingRequiredField("src_ip"))?
        )?;

        let dst_ip = parse_ipv4(
            dst_ip.ok_or(ParseError::MissingRequiredField("dst_ip"))?
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
