/////////////////////////////////////////////////////////
/// ipv4.rs
/// Struct pour parser une adresse IP
/////////////////////////////////////////////////////////


#[derive(
    Debug, Clone, Copy, PartialEq, Eq
)]

/// Definition de la struct IP
pub struct Ipv4Addr {
    pub octets: [
        u8; 
        4
    ],
}