///////////////////////////////////////////////////
/// https://networklessons.com/cisco/ccna-routing-switching-icnd1-100-105/ipv4-packet-header
/// 
/// Structure presente a header IPv4.

use alloc::vec::Vec as VacNoStd;

/// Definition de l'header IPV4
#[derive(Debug, Clone)]
pub struct Ipv4Header {
    pub version: u8,            
    pub ihl: u8,                
    pub dscp: u8,               
    pub total_length: u16,      
    pub identification: u16,    
    pub flags: u8,              
    pub fragment_offset: u16,   
    pub ttl: u8,                
    pub protocol: u8,           
    pub header_checksum: u16,   
    pub src_addr: [u8; 4],      
    pub dst_addr: [u8; 4],      
    pub options: Option<VacNoStd<u8>>,
}
