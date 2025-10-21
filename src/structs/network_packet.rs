use crate::structs::ethernet::EthernetHeader;
use crate::structs::ip::Ipv4Header;
use crate::structs::l4_protocol::L4Data;

#[derive(Debug, Clone)]
pub struct NetworkPacket {
    pub ethernet: EthernetHeader,
    pub ipv4: Ipv4Header,
    pub l4_data: L4Data,
}