use crate::prelude::*;

/// Struct constructeur fichiers PCAP
pub struct PcapWriter {
    pub buffer: VecNoStd<u8>,
}  

/// Read fichiers PCAP
pub struct PcapReader {
    pub data: VecNoStd<u8>,
    pub position: usize,
}