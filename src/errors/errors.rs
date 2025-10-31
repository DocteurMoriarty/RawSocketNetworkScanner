/////////////////////////////////////////////////////////////////
/// errors.rs
/// Definition des erreurs de parsing reseau personnalise
//////////////////////////////////////////////////////////////////

use crate::prelude::*;
use alloc::format;

#[cfg(feature = "std")]
use std::error::Error;

/// Resultat format parsing reseau
pub type Result<T> = core::result::Result<T, ParseError>;

/// Erreur de parsing reseau personnalise 
#[derive(
    Debug, 
    PartialEq, 
    Eq
)]
pub enum ParseError {
    InvalidMac,
    InvalidIpv4,
    TooManyOctets,
    NotEnoughOctets,
    InvalidHex,
    InvalidLength,
    InvalidLengthBytes { 
        size: usize 
    },
    ValueTooLarge { 
        value: u64, 
        size: usize 
    },
    MissingRequiredField(
        &'static str
    ),
    InvalidFormat(
        &'static str
    ),
    IoError(
        StringNoStd
    ),
    JsonError(
        StringNoStd
    ),
    SerdeError(
        StringNoStd
    ),
}

/// Affichage des erreur lisible de parsing reseau explicite
impl fmt::Display for ParseError {
    fn fmt(
        &self, 
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        match self {
            ParseError::InvalidMac => write!(
                f, 
                "Invalid MAC address"
            ),
            ParseError::InvalidIpv4 => write!(
                f, 
                "Invalid IPv4 address"
            ),
            ParseError::TooManyOctets => write!(
                f,
                "Too many octets in IPv4"
            ),
            ParseError::NotEnoughOctets => write!(
                f, 
                "Not enough octets in IPv4"
            ),
            ParseError::InvalidHex => write!(
                f, 
                "Invalid hex value"
            ),
            ParseError::InvalidLength => write!(
                f, 
                "Invalid length"
            ),
            ParseError::InvalidLengthBytes { 
                size 
            } => write!(
                f, 
                "Invalid length bytes size={}", 
                size
            ),
            ParseError::ValueTooLarge { 
                value, 
                size 
            } => write!(
                f, 
                "Value {} too large for size {}", 
                value, 
                size
            ),
            ParseError::MissingRequiredField(
                name
            ) => write!(
                f, 
                "Missing required field {}", 
                name
            ),
            ParseError::InvalidFormat(
                msg
            ) => write!(
                f, 
                "Invalid format: {}", 
                msg
            ),
            ParseError::IoError(
                msg
            ) => write!(
                f, 
                "IO error: {}", 
                msg
            ),
            ParseError::JsonError(
                msg
            ) => write!(
                f, 
                "JSON error: {}", 
                msg
            ),
            ParseError::SerdeError(
                msg
            ) => write!(
                f, 
                "Serde error: {}", 
                msg
            ),
        }
    }
}



// Implementation de From pour convertir une erreur deserde_json_core en une erreur personalisee
// Serialization
impl From<serde_json_core::ser::Error> for ParseError {
    fn from(err:
        serde_json_core::ser::Error
    ) -> Self {
        ParseError::SerdeError(
            format!("Serialization error: {:?}", err)
        )
    }
}
// Deserialization
impl From<serde_json_core::de::Error> for ParseError {
    fn from(err:
        serde_json_core::de::Error
    ) -> Self {
        ParseError::SerdeError(
            format!("Deserialization error: {:?}", err)
        )
    }
}

#[cfg(feature = "std")]
impl Error for ParseError {}