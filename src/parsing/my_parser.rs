use crate::structs::ipv4::Ipv4Addr;
use crate::errors::errors::ParseError;

/// Functions of parsing

/// Parse une MAC address string -> [u8; 6]
pub fn parse_mac(
    mac: &str
) ->
 Result<
    [u8; 6], 
    ParseError
> 
{
    let mut b = [
        0u8; 
        6
    ];
    let mut i = 0;
    for p in mac.split(':') {
        if i >= 6 || p.len() != 2 { 
            return Err(
                ParseError::InvalidMac
            );
        }
        b[i] = u8::from_str_radix(
            p, 
            16
        )
        .map_err(
            |_
            | ParseError::InvalidHex
        )?;
        i += 1;
    }
    if i != 6 { 
        return Err(
            ParseError::InvalidMac
        )
    ;}
    Ok(b)
}

/// parse une IPv4 string -> Ipv4Addr
pub fn parse_ipv4(
    ip: &str
) -> 
Result<
    Ipv4Addr, ParseError
> 
{
    let mut octets = [0u8; 4];
    let mut i = 0;

    for p in ip.split('.') {
        if i >= 4 {
            return Err(
                ParseError::TooManyOctets
            );
        }

        octets[i] = p.parse::<u8>().map_err(|_| ParseError::InvalidIpv4)?;
        i += 1;
    }

    if i != 4 {
        return Err(
            ParseError::NotEnoughOctets
    );
    }

    Ok(
        Ipv4Addr { 
            octets 
        }
    )
}


/// parse une hex string -> u8
pub fn parse_hex(
    octets: &str
) -> 
Result<
    u8, 
    ParseError
> 
{
    u8::from_str_radix(
        octets.trim_start_matches(
            "0x"
        ), 
        16
    )
    .map_err(
    |_
        |ParseError::InvalidHex
    )
}