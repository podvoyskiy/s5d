use std::net::{IpAddr, TcpStream};

use crate::prelude::*;

pub fn to_bytes(addr: &TcpStream) -> Result<Vec<u8>, AppError> {
    let addr = addr.local_addr()?;

    let mut ip_as_bytes = match addr.ip() {
        IpAddr::V4(ipv4) => ipv4.octets().to_vec(),
        IpAddr::V6(ipv6) => ipv6.octets().to_vec(),
    };
    let port_as_bytes: [u8; 2] = addr.port().to_be_bytes();
    ip_as_bytes.extend_from_slice(&port_as_bytes);

    Ok(ip_as_bytes)
}