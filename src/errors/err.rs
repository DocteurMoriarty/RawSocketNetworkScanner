use displaydoc::Display;
use thiserror::Error;

pub type Result
<
    T
> = core::result::Result
<
    T, 
    ParseError
>;

#[derive(
    Display, 
    Debug, 
    Error, 
    PartialEq, 
    Eq
)]
pub enum ParseError {
    /// Invalid MAC address
    InvalidMac,
    /// Invalid IPv4 address
    InvalidIpv4,
    /// Too many octets in IPv4
    TooManyOctets,
    /// Not enough octets in IPv4
    NotEnoughOctets,
    /// Invalid hex value
    InvalidHex,
    /// Invalid length
    InvalidLength,
}