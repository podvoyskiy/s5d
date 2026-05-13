use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::{AppError, consts, utils};

pub enum Atyp {
    DomainName((String, u16)),
    IpV4(Ipv4Addr),
    Ipv6(Ipv6Addr),
}

impl Atyp {
    pub fn to_byte(&self) -> u8 {
        match self {
            Atyp::DomainName(_) => consts::connect::ATYP_DOMAINNAME,
            Atyp::IpV4(_) => consts::connect::ATYP_IPV4,
            Atyp::Ipv6(_) => consts::connect::ATYP_IPV6,
        }
    }
}

//TODO для &str тоже сделать так же через дженерик
impl TryFrom<String> for Atyp {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(value) = value.parse::<IpAddr>() {
            return match value {
                IpAddr::V4(value) => Ok(Atyp::IpV4(value)),
                IpAddr::V6(value) => Ok(Atyp::Ipv6(value)),
            }
        }

        match utils::parse_url(&value) {
            Ok((host, port)) => Ok(Atyp::DomainName((host, port))),
            Err(_) => Err(AppError::InvalidDomain),
        }
    }
}