use crate::prelude::*;
use core::fmt::Write;
use crate::errors::errors::Result;

/// Convertit une adresse MAC en chaîne de caractères formatée
pub fn mac_to_string(
    mac: &[u8; 6]
) -> Result<StringNoStd> {
    let mut s = StringNoStd::new();
    write!(&mut s,
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    ).map_err(
        |_
        | crate::errors::errors::ParseError::InvalidFormat("Fail to format MAC"))?;
    Ok(s)
}
