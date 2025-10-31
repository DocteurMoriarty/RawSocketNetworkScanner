use crate::prelude::StringNoStd;
use alloc::format;

// Formate une adresse MAC en chaîne hexadécimale MAC
pub fn format_mac(mac: &[u8; 6]) -> StringNoStd {
    format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", 
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5])
}

// Formate une adresse IP en chaîne décimale
pub fn format_ip(ip: &[u8; 4]) -> StringNoStd {
    format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
}

// Formate un tableau d'octets en une chaîne de représentation hexadécimale
pub fn format_bytes(bytes: &[u8]) -> StringNoStd {
    let mut result = StringNoStd::new();
    for (i, &byte) in bytes.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(&format!("{:02X}", byte));
    }
    result
}

// Obtient le timestamp actuel en millisecondes
pub fn get_timestamp_ms() -> u64 {
    0
}
