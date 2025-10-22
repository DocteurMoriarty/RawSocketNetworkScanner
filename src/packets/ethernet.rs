use alloc::vec;
use alloc::vec::Vec;
use crate::{
    utils::{
        convert_bytes::convert_n_to_bytes,
        push_bytes::push_bytes,
    },
    structs::ethernet::EthernetHeader,
    errors::errors::Result,
};

pub fn pack_ethernet(
    header: &EthernetHeader, 
    payload: &[u8]
) -> 
Result<
    Vec<u8>
    > 
{
    let total_len 
        = 14 + payload.len();
    let mut packet 
        = vec![
            0u8; total_len
        ];
    let mut offset = 0;

    let ethertype_bytes = convert_n_to_bytes(
        header.ethertype as u64, 
        2
    )?;

    offset = push_bytes(
        &mut packet, 
        offset, 
        &header.dst_mac
    );
    offset = push_bytes(
        &mut packet, 
        offset, 
        &header.src_mac
    );

    offset = push_bytes(
        &mut packet, 
        offset, 
        &ethertype_bytes
    );

    offset = push_bytes(
        &mut packet, 
        offset, 
        payload
    );

    Ok(
        packet
    )
}
