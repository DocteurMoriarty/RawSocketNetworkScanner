use crate::structs::{
    tcp::TcpHeader,
    udp::UdpHeader
};

/// Definition des donnees de couche 4
/// Contient TCP ou UDP.
#[derive(Debug, Clone)]
pub enum L4Data {
    Tcp(
        TcpHeader
    ),
    Udp(
        UdpHeader
    ),
}


/// Definition des protocoles de couche 4
/// Type de protocole
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum L4Protocol {
    Tcp,
    Udp,
} 

