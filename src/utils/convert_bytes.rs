use crate::errors::err::{
    Result,
    ParseError::InvalidLengthBytes
};

pub fn convert_n_to_bytes <T: Into<u64>>(value: T, size: usize) -> Result<Vec<u8>> {
    if size != 2 
        && size != 4 
        && size != 8 
    {
        return Err(
            InvalidLengthBytes { 
                size: (
                    size
                ) 
            }
        );
    }
    let mut bytes = vec![
        0u8; 
        size
    ];
    let value = value.into();
    for i in 0..size {
        bytes[
            size - 1 - i
        ] = (
            (
                value >> (
                    i * 8
                )
            ) & 0xFF
        ) as u8;
    }
    Ok(bytes)
}