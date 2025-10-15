use crate::{
    utils::{
        convert_bytes::convert_n_to_bytes,
        push_bytes::push_bytes
    },
    structs::tcp::TcpHeader,
    errors::err::{
        Result,
        ParseError
    }
};

/*
pub fn pack_tcp(header: &TcpHeader) -> Result<Vec<u8>> {
    let payload = header.payload.unwrap_or(&[]);

}
 */

pub fn pack_tcp(header: &TcpHeader) -> Result<Vec<u8>> {
    let payload: &[
        u8
    ];
    if let Some(
        p
    ) = header.payload {
        payload = p;
    } 
    else 
    {
        payload = &[];
    }
 
    let total_len = 20 + payload.len();
 
    let mut packet = vec![
        0u8; 
        total_len
    ];
    
    let mut offset = 0;

    let fields = [
        header.src_port as u64,
        header.dst_port as u64,
        header.sequence_number as u64,
        header.ack_nowledgment_number as u64,
        header.data_offset as u64,
        header.window as u64,
        header.checksum as u64,
        header.urgent_pointer as u64,
    ];

    
    let sizes = [2, 2, 4, 4, 2, 2, 2, 2];
    for (
        field, 
        &size
    ) 
    in fields.iter()
    .zip(
        &sizes
    ) 
    {
        offset = push_bytes(
            &mut packet, 
            offset, 
            &convert_n_to_bytes(
                *field, 
                size
            )
            ?
        );
    }

    offset = push_bytes(
        &mut packet, 
        offset, 
        payload
    );

    Ok(
        packet
    )
}
