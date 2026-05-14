use std::{net::{SocketAddr, SocketAddrV4, SocketAddrV6}, str::FromStr};

use crate::{AppError, consts, utils};

#[derive(Debug)]
pub enum Atyp {
    Domain((String, u16)),
    Ipv4(SocketAddrV4),
    Ipv6(SocketAddrV6),
}

impl Atyp {
    pub fn u8(&self) -> u8 {
        match self {
            Atyp::Domain(_) => consts::connect::ATYP_DOMAINNAME,
            Atyp::Ipv4(_) => consts::connect::ATYP_IPV4,
            Atyp::Ipv6(_) => consts::connect::ATYP_IPV6,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Atyp::Domain((host, port)) => {
                let mut bytes: Vec<u8> = Vec::with_capacity(1 + 1 + host.len() + 2);
                bytes.push(self.u8());
                bytes.push(host.len() as u8);
                bytes.extend_from_slice(host.as_bytes());
                bytes.extend(port.to_be_bytes());
                bytes
            },
            Atyp::Ipv4(socket_addr) => {
                let mut bytes: Vec<u8> = Vec::with_capacity(1 + 4 + 2);
                bytes.push(self.u8());
                bytes.extend(socket_addr.ip().to_bits().to_be_bytes());
                bytes.extend(socket_addr.port().to_be_bytes());
                bytes
            },
            Atyp::Ipv6(socket_addr) => {
                let mut bytes: Vec<u8> = Vec::with_capacity(1 + 16 + 2);
                bytes.push(self.u8());
                bytes.extend(socket_addr.ip().to_bits().to_be_bytes());
                bytes.extend(socket_addr.port().to_be_bytes());
                bytes
            },
        }
    }
}

impl FromStr for Atyp {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<SocketAddr>() {
            return Ok(match value {
                SocketAddr::V4(socket_addr_v4) => Atyp::Ipv4(socket_addr_v4),
                SocketAddr::V6(socket_addr_v6) => Atyp::Ipv6(socket_addr_v6),
            });
        }

        match utils::parse_url(s) {
            Ok((host, port)) => Ok(Atyp::Domain((host, port))),
            Err(_) => Err(AppError::InvalidDomain),
        }
    }
}