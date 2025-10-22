use alloc::vec;
use alloc::vec::Vec;
use crate::{
    utils::{
        convert_bytes::convert_n_to_bytes,
        push_bytes::push_bytes
    },
    structs::tcp::TcpHeader,
    errors::errors::Result,
};




pub fn pack_tcp(header: &TcpHeader) -> Result<Vec<u8>> {
    let payload: &[u8];
    if let Some(p) = &header.payload {
        payload = p;
    } else {
        payload = &[];
    }

    let options: &[u8];
    if let Some(
        o
    ) = &header.options {
        options = &o[..];
    } 
    else 
    {
        options = &[];
    }

    let total_len = 20 
        + options.len() 
        + payload.len();

    let mut packet = vec![
        0u8; 
        total_len
    ];

    let mut offset = 0;

    let data_offset_value = (
        (
            20 + options.len()
        ) 
        / 4
    ) as u16;
    
    let combined_offset_reserved_flags: u16 = (
            data_offset_value << 12
        )
        | (
            (
                header.reserved as u16
            ) << 9
        )
        | (
            header.flags & 0x01FF
        );


    let fields = [
        (header.src_port as u64, 2),
        (header.dst_port as u64, 2),
        (header.sequence_number as u64, 4),
        (header.ack_nowledgment_number as u64, 4),
        (combined_offset_reserved_flags as u64, 2),
        (header.window as u64, 2),
        (header.checksum as u64, 2),
        (header.urgent_pointer as u64, 2),
    ];


    for &(field, size) in &fields {
        let bytes = convert_n_to_bytes(field, size)?;
        offset = push_bytes(&mut packet, offset, &bytes);
    }

    if !options.is_empty() {
        offset = push_bytes(&mut packet, offset, options);
    }

    if !payload.is_empty() {
        offset = push_bytes(&mut packet, offset, payload);
    }

    Ok(packet)
}
