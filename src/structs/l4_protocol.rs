use crate::structs::tcp::TcpHeader;
use crate::structs::udp::UdpHeader;

#[derive(Debug, Clone)]
pub enum L4Data {
    Tcp(TcpHeader),
    Udp(UdpHeader),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum L4Protocol {
    Tcp,
    Udp,
}