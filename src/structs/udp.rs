///////////////////////////////////////////////
/// UDP Header Structure
/// https://tools.ietf.org/html/rfc768
/// Structure presente a header UDP.
/// https://digilent.com/blog/udp-vs-tcp/?srsltid=AfmBOoovDA4GmltoRpmhQjs-iM-RcGPEK3JPuLoohfII9ozCzOltMeN5
/// ///////////////////////////////////////////


#[derive(Debug, Clone, Copy)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}
