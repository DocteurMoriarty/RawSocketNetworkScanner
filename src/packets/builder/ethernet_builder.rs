///////////////////////////////////////////////////////////////////////////
/// ethernet_builder.rs
/// Constructeur de l'header Ethernet
///////////////////////////////////////////////////////////////////////////

use crate::structs::ethernet::EthernetHeader;


pub struct EthernetBuilder;

impl EthernetBuilder {
    /// Constructeur de l'header Ethernet
    pub fn new() -> Self {
        Self
    }

    /// Construit l'header Ethernet avec les MAC
    pub fn build_ethernet_header(
        &self, 
        src_mac: [u8; 6]
        , dst_mac: [u8; 6]
    ) -> EthernetHeader {
        EthernetHeader {
            dst_mac,
            src_mac,
            ethertype: 0x0800,
        }
    }
}