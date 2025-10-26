use crate::prelude::*;

pub struct PcapWriter {
    pub buffer: VecNoStd<u8>,
}  

pub struct PcapReader {
    pub data: VecNoStd<u8>,
    pub position: usize,
}