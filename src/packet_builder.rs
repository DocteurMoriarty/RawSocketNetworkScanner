use crate::{
    structs::{
        ethernet::EthernetHeader,
        ip::Ipv4Header,
        tcp::TcpHeader,
        udp::UdpHeader,
        ipv4::Ipv4Addr,
    },
    packets::{
        ethernet::pack_ethernet,
        ip::pack_ipv4,
        l4::{tcp::pack_tcp, udp::pack_udp},
    },
    utils::checksum::internet_checksum,
    errors::errors::Result,
};

/// Représente un paquet réseau complet avec tous ses en-têtes
#[derive(Debug, Clone)]
pub struct NetworkPacket {
    pub ethernet: EthernetHeader,
    pub ipv4: Ipv4Header,
    pub l4_data: L4Data,
}

/// Représente les données de couche 4 (TCP ou UDP)
#[derive(Debug, Clone)]
pub enum L4Data {
    Tcp(TcpHeader),
    Udp(UdpHeader),
}

/// Paramètres pour construire un paquet réseau
#[derive(Debug, Clone)]
pub struct PacketBuilder {
    pub src_ip: Ipv4Addr,
    pub dst_ip: Ipv4Addr,
    pub src_mac: [u8; 6],
    pub dst_mac: [u8; 6],
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: L4Protocol,
    pub ip_bitfield: u8,
    pub payload: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum L4Protocol {
    Tcp,
    Udp,
}

impl PacketBuilder {
    /// Construit un paquet réseau complet
    pub fn build_packet(&self) -> Result<NetworkPacket> {
        // 1. Construire l'en-tête de couche 4 (TCP ou UDP)
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

        // 2. Construire l'en-tête IPv4
        let ipv4_header = self.build_ipv4_header(&l4_data)?;

        // 3. Construire l'en-tête Ethernet
        let ethernet_header = self.build_ethernet_header();

        Ok(NetworkPacket {
            ethernet: ethernet_header,
            ipv4: ipv4_header,
            l4_data,
        })
    }

    /// Construit l'en-tête TCP
    fn build_tcp_header(&self) -> Result<TcpHeader> {
        let payload = self.payload.clone();
        let payload_len = payload.as_ref().map(|p| p.len()).unwrap_or(0);

        let mut tcp_header = TcpHeader {
            src_port: self.src_port,
            dst_port: self.dst_port,
            sequence_number: 0, // Peut être configuré
            ack_nowledgment_number: 0,
            data_offset: 5, // 5 * 4 = 20 bytes
            reserved: 0,
            flags: 0x02, // SYN flag par défaut
            window: 65535,
            checksum: 0, // Sera calculé plus tard
            urgent_pointer: 0,
            options: None,
            payload,
        };

        // Calculer le checksum TCP
        tcp_header.checksum = self.calculate_tcp_checksum(&tcp_header)?;

        Ok(tcp_header)
    }

    /// Construit l'en-tête UDP
    fn build_udp_header(&self) -> Result<UdpHeader> {
        let payload = self.payload.clone();
        let payload_len = payload.as_ref().map(|p| p.len()).unwrap_or(0);
        
        // Calculer la longueur totale du datagramme UDP
        let udp_length = 8 + payload_len; // 8 bytes pour l'en-tête UDP

        let mut udp_header = UdpHeader {
            src_port: self.src_port,
            dst_port: self.dst_port,
            length: udp_length as u16,
            checksum: 0, // Sera calculé plus tard
            payload,
        };

        // Calculer le checksum UDP
        udp_header.checksum = self.calculate_udp_checksum(&udp_header)?;

        Ok(udp_header)
    }

    /// Construit l'en-tête IPv4
    fn build_ipv4_header(&self, l4_data: &L4Data) -> Result<Ipv4Header> {
        let l4_length = match l4_data {
            L4Data::Tcp(tcp) => {
                let payload_len = tcp.payload.as_ref().map(|p| p.len()).unwrap_or(0);
                20 + payload_len // 20 bytes pour l'en-tête TCP + payload
            }
            L4Data::Udp(udp) => {
                let payload_len = udp.payload.as_ref().map(|p| p.len()).unwrap_or(0);
                8 + payload_len // 8 bytes pour l'en-tête UDP + payload
            }
        };

        let total_length = 20 + l4_length; // 20 bytes pour l'en-tête IPv4 + données L4

        let mut ipv4_header = Ipv4Header {
            version: 4,
            ihl: 5, // 5 * 4 = 20 bytes
            dscp: 0,
            total_length: total_length as u16,
            identification: 0, // Peut être généré aléatoirement
            flags: (self.ip_bitfield >> 5) & 0x07, // Extraire les 3 bits de flags
            fragment_offset: ((self.ip_bitfield & 0x1F) as u16) << 8, // Extraire les 5 bits de fragment offset
            ttl: 64,
            protocol: match self.protocol {
                L4Protocol::Tcp => 6,
                L4Protocol::Udp => 17,
            },
            header_checksum: 0, // Sera calculé plus tard
            src_addr: self.src_ip.octets,
            dst_addr: self.dst_ip.octets,
            options: None,
        };

        // Calculer le checksum IPv4
        ipv4_header.header_checksum = self.calculate_ipv4_checksum(&ipv4_header)?;

        Ok(ipv4_header)
    }

    /// Construit l'en-tête Ethernet
    fn build_ethernet_header(&self) -> EthernetHeader {
        EthernetHeader {
            dst_mac: self.dst_mac,
            src_mac: self.src_mac,
            ethertype: 0x0800, // IPv4
        }
    }

    /// Calcule le checksum TCP
    fn calculate_tcp_checksum(&self, tcp_header: &TcpHeader) -> Result<u16> {
        // Créer un pseudo-en-tête pour le calcul du checksum TCP
        let mut pseudo_header = Vec::new();
        
        // Adresse source IP (4 bytes)
        pseudo_header.extend_from_slice(&self.src_ip.octets);
        // Adresse destination IP (4 bytes)
        pseudo_header.extend_from_slice(&self.dst_ip.octets);
        // Zéro (1 byte)
        pseudo_header.push(0);
        // Protocole TCP (1 byte)
        pseudo_header.push(6);
        // Longueur du segment TCP (2 bytes)
        let tcp_length = 20 + tcp_header.payload.as_ref().map(|p| p.len()).unwrap_or(0);
        pseudo_header.extend_from_slice(&(tcp_length as u16).to_be_bytes());

        // Construire le segment TCP pour le calcul du checksum
        let tcp_segment = pack_tcp(tcp_header)?;
        
        // Combiner pseudo-en-tête et segment TCP
        let mut checksum_data = pseudo_header;
        checksum_data.extend_from_slice(&tcp_segment);

        Ok(internet_checksum(&checksum_data))
    }

    /// Calcule le checksum UDP
    fn calculate_udp_checksum(&self, udp_header: &UdpHeader) -> Result<u16> {
        // Créer un pseudo-en-tête pour le calcul du checksum UDP
        let mut pseudo_header = Vec::new();
        
        // Adresse source IP (4 bytes)
        pseudo_header.extend_from_slice(&self.src_ip.octets);
        // Adresse destination IP (4 bytes)
        pseudo_header.extend_from_slice(&self.dst_ip.octets);
        // Zéro (1 byte)
        pseudo_header.push(0);
        // Protocole UDP (1 byte)
        pseudo_header.push(17);
        // Longueur du datagramme UDP (2 bytes)
        pseudo_header.extend_from_slice(&udp_header.length.to_be_bytes());

        // Construire le datagramme UDP pour le calcul du checksum
        let udp_datagram = pack_udp(udp_header)?;
        
        // Combiner pseudo-en-tête et datagramme UDP
        let mut checksum_data = pseudo_header;
        checksum_data.extend_from_slice(&udp_datagram);

        Ok(internet_checksum(&checksum_data))
    }

    /// Calcule le checksum IPv4
    fn calculate_ipv4_checksum(&self, ipv4_header: &Ipv4Header) -> Result<u16> {
        // Construire l'en-tête IPv4 sans le checksum pour le calcul
        let mut temp_header = ipv4_header.clone();
        temp_header.header_checksum = 0;
        
        let ip_header_bytes = pack_ipv4(&temp_header, &[])?;
        
        // Prendre seulement les 20 premiers bytes (en-tête IPv4 de base)
        let header_for_checksum = &ip_header_bytes[..20];
        
        Ok(internet_checksum(header_for_checksum))
    }
}

impl NetworkPacket {
    /// Assemble le paquet complet en bytes
    pub fn assemble_packet(&self) -> Result<Vec<u8>> {
        // 1. Construire les données de couche 4
        let l4_data = match &self.l4_data {
            L4Data::Tcp(tcp_header) => pack_tcp(tcp_header)?,
            L4Data::Udp(udp_header) => pack_udp(udp_header)?,
        };

        // 2. Construire l'en-tête IPv4 avec les données L4 comme payload
        let ip_packet = pack_ipv4(&self.ipv4, &l4_data)?;

        // 3. Construire le paquet Ethernet complet avec le paquet IP comme payload
        let ethernet_packet = pack_ethernet(&self.ethernet, &ip_packet)?;

        Ok(ethernet_packet)
    }

    /// Obtient la taille totale du paquet
    pub fn get_packet_size(&self) -> usize {
        let l4_size = match &self.l4_data {
            L4Data::Tcp(tcp) => 20 + tcp.payload.as_ref().map(|p| p.len()).unwrap_or(0),
            L4Data::Udp(udp) => 8 + udp.payload.as_ref().map(|p| p.len()).unwrap_or(0),
        };
        
        14 + 20 + l4_size // Ethernet (14) + IPv4 (20) + L4
    }
}

/// Fonction utilitaire pour créer un PacketBuilder à partir des arguments CLI
impl PacketBuilder {
    pub fn from_cli_args(
        src_ip: Option<String>,
        dst_ip: Option<String>,
        src_mac: Option<[u8; 6]>,
        dst_mac: Option<[u8; 6]>,
        src_port: Option<u16>,
        dst_port: Option<u16>,
        l4_protocol: Option<String>,
        ip_bitfield: Option<u8>,
        payload: Option<Vec<u8>>,
    ) -> Result<Self> {

        let src_ip = src_ip
            .ok_or_else(|| crate::errors::errors::ParseError::MissingRequiredField("src_ip"))?
            .parse::<std::net::Ipv4Addr>()
            .map_err(|_| crate::errors::errors::ParseError::InvalidIpv4)?;
        
        let dst_ip = dst_ip
            .ok_or_else(|| crate::errors::errors::ParseError::MissingRequiredField("dst_ip"))?
            .parse::<std::net::Ipv4Addr>()
            .map_err(|_| crate::errors::errors::ParseError::InvalidIpv4)?;

        let src_mac = src_mac.unwrap_or([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let dst_mac = dst_mac.unwrap_or([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]); // Broadcast par défaut

        let src_port = src_port.unwrap_or(12345); // Port source par défaut
        let dst_port = dst_port.unwrap_or(80); // Port destination par défaut

        let protocol = match l4_protocol.as_deref() {
            Some("tcp") => L4Protocol::Tcp,
            Some("udp") => L4Protocol::Udp,
            _ => L4Protocol::Tcp, // TCP par défaut
        };

        let ip_bitfield = ip_bitfield.unwrap_or(0x00);

        Ok(PacketBuilder {
            src_ip: Ipv4Addr { octets: src_ip.octets() },
            dst_ip: Ipv4Addr { octets: dst_ip.octets() },
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
