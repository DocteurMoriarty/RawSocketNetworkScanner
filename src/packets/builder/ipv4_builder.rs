use crate::{
    structs::{
        ip::Ipv4Header,
        ipv4::Ipv4Addr,
        l4_protocol::L4Data,
    },
    packets::ip::pack_ipv4,
    utils::{
        checksum::internet_checksum,
        payload_size::payload_len
    },
    errors::errors::Result,
};

pub struct Ipv4Builder {
    src_ip: Ipv4Addr,
    dst_ip: Ipv4Addr,
    ip_bitfield: u8,
}

// Implementation de Ipv4Builder
impl Ipv4Builder {
    pub fn new(
        src_ip: Ipv4Addr, 
        dst_ip: Ipv4Addr, 
        ip_bitfield: u8
    ) -> Self {
        Self {
            src_ip,
            dst_ip,
            ip_bitfield,
        }
    }

    /// Construit l'header IPv4
    pub fn build_ipv4_header(
        &self, 
        l4_data: &L4Data
    ) -> 
    Result<
        Ipv4Header
    > {
        let l4_length = match l4_data {
            L4Data::Tcp(tcp) => payload_len(&tcp.payload) + 20,
            L4Data::Udp(udp) => payload_len(&udp.payload) + 8,
        };
        let total_length = 20 + l4_length;
        let mut ipv4_header = Ipv4Header {
            version: 4,
            ihl: 5,
            dscp: 0,
            total_length: total_length as u16,
            identification: 0,
            flags: (
                self.ip_bitfield >> 5
            ) & 0x07,
            fragment_offset: (
                (
                    self.ip_bitfield & 0x1F
                ) as u16
            ) << 8,
            ttl: 64,
            protocol: match l4_data {
                L4Data::Tcp(
                    _
                ) => 6,
                L4Data::Udp(
                    _
                ) => 17,
            },
            header_checksum: 0,
            src_addr: self.src_ip.octets,
            dst_addr: self.dst_ip.octets,
            options: None,
        };

        ipv4_header.header_checksum = self.calculate_ipv4_checksum(
            &ipv4_header
        )?;

        Ok(
            ipv4_header
        )
    }

    /// Calcule le checksum de l'header IPv4
    fn calculate_ipv4_checksum(
        &self, 
        ipv4_header: &Ipv4Header
    ) -> 
    Result<
        u16
    > {
        let mut temp_header = ipv4_header.clone();
        temp_header.header_checksum = 0;
        let ip_header_bytes = pack_ipv4(&temp_header, &[])?;
        let header_for_checksum = &ip_header_bytes[..20];
        Ok(
            internet_checksum(
                header_for_checksum
            )
        )
    }
}
