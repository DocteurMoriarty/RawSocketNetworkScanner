/// Fonction utilitaire pour convertir un nombre en un vecteur d’octets de taille spécifiée (2, 4 ou 8 octets)
/// en format Big Endian.
/// Cette fonction renvoie une erreur personnaliser
/// Cette fonction a est develloper pour eviter l’utilisation de fonctions haut niveau
/// et de la librairie standard std, comme to_be_bytes ou copy_from_slice

use crate::prelude::*;

use crate::errors::errors::{
    Result,
    ParseError::{
        InvalidLengthBytes,
        ValueTooLarge
    }
};

/// Convertit un nombre en un vecteur d’octets de taille spécifiée (2, 4 ou 8 octets) en format Big Endian.
pub fn convert_n_to_bytes <T: Into<u64>>(value: T, size: usize) -> Result<VecNoStd<u8>> {
    
    if size == 1 {
        let byte = (value.into() & 0xFF) as u8;
        return Ok(vec![byte]);
    }

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

    if size < 8 
    && value >= (
        1u64 << (
            size * 8
        )
    )
    {
        return Err(
            ValueTooLarge {
                value, 
                size 
            }
        );
    }


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