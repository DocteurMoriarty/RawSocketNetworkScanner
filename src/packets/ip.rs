use crate::{
    utils::{
        convert_bytes::convert_n_to_bytes,
        push_bytes::push_bytes,
    },
    structs::ip::Ipv4Header,
    errors::errors::Result,
};

pub fn pack_ipv4(
    header: &Ipv4Header, 
    payload: &[u8]
) -> 
Result<
    Vec<u8>
> 
{
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

    let header_len = (
        header.ihl * 4
    ) as usize;
    let total_len 
        = header_len + payload.len();

    let mut packet = vec![
        0u8; total_len
    ];
    let mut offset = 0;

    let version_ihl = (
        header.version << 4
    ) | header.ihl;
    let flags_fragment = (
        (
            header.flags as u16
        ) << 13
    ) | (
        header.fragment_offset & 0x1FFF
    );

    let fields = [
        (version_ihl as u64, 1),
        (header.dscp as u64, 1),
        (header.total_length as u64, 2),
        (header.identification as u64, 2),
        (flags_fragment as u64, 2),
        (header.ttl as u64, 1),
        (header.protocol as u64, 1),
        (header.header_checksum as u64, 2),
    ];

    for &(field, size) in &fields {
        let bytes = convert_n_to_bytes(field, size)?;
        offset = push_bytes(&mut packet, offset, &bytes);
    }

    offset = push_bytes(&mut packet, offset, &header.src_addr);
    offset = push_bytes(&mut packet, offset, &header.dst_addr);

    if !options.is_empty() {
        offset = push_bytes(&mut packet, offset, options);
    }

    offset = push_bytes(&mut packet, offset, payload);

    Ok(packet)
}
