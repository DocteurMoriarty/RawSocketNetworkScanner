use alloc::vec::Vec;
///////////////////////////////////////////////
/// UDP Header Structure
/// https://tools.ietf.org/html/rfc768
/// Structure presente a header UDP.
/// https://digilent.com/blog/udp-vs-tcp/?srsltid=AfmBOoovDA4GmltoRpmhQjs-iM-RcGPEK3JPuLoohfII9ozCzOltMeN5
/// ///////////////////////////////////////////

/// Definition de l'header UDP
/// Contient les infos du header.
#[derive(Debug, Clone)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
    pub payload: Option<Vec<u8>>
}
