use alloc::vec;
use alloc::vec::Vec;
use crate::{
    utils::{
        convert_bytes::convert_n_to_bytes,
        push_bytes::push_bytes
    },
    structs::udp::UdpHeader,
    errors::errors::{
        Result
    }
};

/// Emballe un header UDP en un vecteur doctets
pub fn pack_udp(header: &UdpHeader) -> Result<Vec<u8>> {
    let payload: &[u8];
    if let Some(p) = &header.payload {
        payload = p;
    } else {
        payload = &[];
    }

    let total_len = 8 + payload.len();
    
    let mut packet = vec![
        0u8; 
        total_len
    ];

    let mut offset = 0;

    let fields = [
        header.src_port as u64,
        header.dst_port as u64,
        header.length as u64,
        header.checksum as u64,
    ];

    for &field in &fields {
        offset = push_bytes(
            &mut packet, 
            offset, 
            &convert_n_to_bytes(
                field, 
                2
            )
            ?
        );
    }

    push_bytes(
        &mut packet, 
        offset, 
        payload
    );

    Ok(
        packet
    )

}
