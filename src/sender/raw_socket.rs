use crate::errors::errors::Result;
use crate::structs::socket::RawSocketSender;
use core::mem;

// Envoie de paquets via un raw socket
impl RawSocketSender {
    
    // Constructor
    pub fn new() -> Result<Self> {
        let fd = unsafe {
            libc::socket(
                libc::AF_PACKET,
                libc::SOCK_RAW,
                (libc::ETH_P_ALL as u16).to_be() as i32)
            };
        if fd < 0 {
            return Err(
                crate::errors::errors::ParseError::IoError(
                    "socket() failed".into()
                )
            );
        }
        Ok(Self { fd })
    }

    // Definit le timeout décriture du socket
    pub fn set_write_timeout(&self, timeout_ms: Option<u64>) -> Result<()> {
        if let Some(ms) = timeout_ms {
            let tv = libc::timeval {
                tv_sec: (ms / 1000) as libc::time_t,
                tv_usec: ((ms % 1000) * 1000) as libc::suseconds_t,
            };
            let ret = unsafe {
                libc::setsockopt(
                    self.fd,
                    libc::SOL_SOCKET,
                    libc::SO_SNDTIMEO,
                    &tv as *const _ as *const libc::c_void,
                    mem::size_of::<libc::timeval>() as libc::socklen_t,
                )
            };
            if ret < 0 {
                return Err(
                    crate::errors::errors::ParseError::IoError(
                        "setsockopt() failed".into()
                    )
                );
            }
        }
        Ok(())
    }

    pub fn send(&self, if_index: i32, dst_mac: [u8; 6], packet: &[u8]) -> Result<usize> {
        let saddr = libc::sockaddr_ll {
            sll_family: libc::AF_PACKET as u16,
            sll_protocol: (libc::ETH_P_ALL as u16).to_be(),
            sll_ifindex: if_index,
            sll_hatype: 0,
            sll_pkttype: 0,
            sll_halen: 6,
            sll_addr: [
                dst_mac[0], dst_mac[1], dst_mac[2], dst_mac[3], dst_mac[4], dst_mac[5], 0, 0,
            ],
        };
        let ret = unsafe {
            libc::sendto(
                self.fd,
                packet.as_ptr() as *const libc::c_void,
                packet.len(),
                0,
                &saddr as *const _ as *const libc::sockaddr,
                mem::size_of::<libc::sockaddr_ll>() as libc::socklen_t,
            )
        };
        if ret < 0 {
            return Err(
                crate::errors::errors::ParseError::IoError(
                    "sendto() failed".into()
                )
            );
        }
        Ok(
            ret as usize
        )
    }
}

// Implémentation de RawSocketSender
impl Drop for RawSocketSender {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}

// Obtient lindice de linterface réseau par son nom
pub fn get_interface_index(name: &str) -> Result<i32> {
    use alloc::ffi::CString;
    let c_name = CString::new(name).unwrap();
    let fd = unsafe {
        libc::socket(
            libc::AF_PACKET,
            libc::SOCK_RAW,
            (libc::ETH_P_ALL as u16).to_be() as i32)
        };
    if fd < 0 {
        return Err(
            crate::errors::errors::ParseError::IoError(
                "socket() failed".into()
        ));
    }
    let if_index = unsafe {
        libc::if_nametoindex(c_name.as_ptr())
    } as i32;
    unsafe {
        libc::close(fd)
    };
    if if_index <= 0 {
        return Err(
            crate::errors::errors::ParseError::InvalidFormat(
                "invalid interface name"
            )
        );
    }
    Ok(
        if_index
    )
}


