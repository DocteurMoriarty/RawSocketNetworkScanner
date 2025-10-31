use alloc::vec::Vec;
use crate::{
    structs::network_packet::NetworkPacket,
    errors::errors::Result,
};

// Extension de NetworkPacket pour l'assemblage
impl NetworkPacket {
    pub fn assemble_packet(
        &self
    ) -> 
    Result<Vec<u8>> {
        let assembler = super::packet_assembler::PacketAssembler::new();
        assembler.assemble_packet(
            self
        )
    }

    pub fn get_packet_size(
        &self
    ) -> usize {
        let assembler = super::packet_assembler::PacketAssembler::new();
        assembler.get_packet_size(
            self
        )
    }
}
