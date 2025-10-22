//////////////////////////////////////////
/// packet_builder.rs
/// Structure pour construire un paquet complet
//////////////////////////////////////////////

use crate::prelude::*;
use crate::structs::{
    ipv4::Ipv4Addr,
    l4_protocol::L4Protocol,
};


/// Structure pour construire un paquet complet
/// Contient les informations pour construire le paquet
#[derive(Debug, Clone)]
pub struct PacketBuilder {
    pub src_ip: Ipv4Addr,
    pub dst_ip: Ipv4Addr,
    pub src_mac: [u8; 6],
    pub dst_mac: [u8; 6],
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: L4Protocol,
    pub ip_bitfield: u8,
    pub payload: Option<VecNoStd<u8>>,
}