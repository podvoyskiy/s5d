use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, str::FromStr};

use crate::{AppError, consts, utils};

#[derive(Debug)]
pub enum Atyp {
    Domain((String, u16)),
    IpV4(Ipv4Addr),
    IpV6(Ipv6Addr),
}

impl Atyp {
    pub fn to_byte(&self) -> u8 {
        match self {
            Atyp::Domain(_) => consts::connect::ATYP_DOMAINNAME,
            Atyp::IpV4(_) => consts::connect::ATYP_IPV4,
            Atyp::IpV6(_) => consts::connect::ATYP_IPV6,
        }
    }
}

impl FromStr for Atyp {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<IpAddr>() {
            return Ok(match value {
                IpAddr::V4(value) => Atyp::IpV4(value),
                IpAddr::V6(value) => Atyp::IpV6(value),
            })
        }

        match utils::parse_url(s) {
            Ok((host, port)) => Ok(Atyp::Domain((host, port))),
            Err(_) => Err(AppError::InvalidDomain),
        }
    }
}