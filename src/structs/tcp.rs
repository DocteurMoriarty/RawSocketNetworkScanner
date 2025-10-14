#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpHeader <'a>{
    pub src_port: u16,
    pub dst_port: u16,
    pub sequence_number: u32,
    pub ack_nowledgment_number: u32,
    pub data_offset: u8, 
    pub reserved: u8,
    pub flags: u16,
    pub window: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    pub options: Option<Vec<u8>>,
    pub payload: Option<&'a [u8]>
}

