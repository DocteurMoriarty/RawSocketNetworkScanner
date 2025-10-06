// Struct for Parse IP Address
#[derive(
    Debug, 
    Clone, 
    Copy, 
    PartialEq, 
    Eq
)]
pub struct Ipv4Addr {
    pub octets: [
        u8; 
        4
    ],
}