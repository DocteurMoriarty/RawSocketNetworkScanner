////////////////////////////////////////////////
/// https://notes.networklessons.com/ethernet-header
////////////////////////////////////////////////

/// Definition de l'header Ethernet
#[derive(Debug, Clone, Copy)]
pub struct EthernetHeader {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
}
