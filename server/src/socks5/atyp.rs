use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};

use crate::{prelude::*};

#[repr(u8)]
pub enum Atyp { //TODO maybe move it
    IpV4 = consts::connect::ATYP_IPV4,
    DomainName = consts::connect::ATYP_DOMAINNAME,
    Ipv6 = consts::connect::ATYP_IPV6
}

impl Atyp {
    pub fn parse_addrs(&self, buf: &[u8]) -> Result<Vec<SocketAddr>, AppError> {
        match self {
            Atyp::IpV4 => {
                if buf.len() != 6 { return Err(AppError::InvalidIpv4); }
        
                let port = u16::from_be_bytes([buf[4], buf[5]]);
                let addr = SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(buf[0], buf[1], buf[2], buf[3])), 
                    port
                );

                debug!(ip = %addr.ip(), port = addr.port(), "parsed IPv4 address");
                Ok(vec![addr])
            },
            Atyp::DomainName => { // 1 byte is domain length, followed by the domain, then 2 bytes for the port
                let domain_len = *buf.first().ok_or(AppError::InvalidDomain)? as usize;
                let domain_bytes = buf.get(1..1 + domain_len).ok_or(AppError::InvalidDomain)?;
                let port_bytes = buf.get(1 + domain_len..1 + domain_len + 2).ok_or(AppError::InvalidDomain)?;

                let domain = String::from_utf8_lossy(domain_bytes);
                let port = u16::from_be_bytes([port_bytes[0], port_bytes[1]]);

                debug!(%domain, port, "resolving domain name");

                let addrs = (domain.as_ref(), port).to_socket_addrs().map_err(|_| AppError::InvalidDomain)?;

                Ok(addrs.collect())
            },
            Atyp::Ipv6 => {
                if buf.len() != 18 { return Err(AppError::InvalidIpv6); }

                let port = u16::from_be_bytes([buf[16], buf[17]]);
                let ip_bytes: [u8; 16] = buf[0..16].try_into().map_err(|_| AppError::InvalidIpv6)?;
                let addr = SocketAddr::new(
                    IpAddr::V6(Ipv6Addr::from(ip_bytes)),
                    port
                );

                debug!(ip = %addr.ip(), port = addr.port(), "parsed IPv6 address");
                Ok(vec![addr])
            }
        }
    }
}

impl TryFrom<u8> for Atyp {
    type Error = AppError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            consts::connect::ATYP_IPV4 => Ok(Self::IpV4),
            consts::connect::ATYP_DOMAINNAME => Ok(Self::DomainName),
            consts::connect::ATYP_IPV6 => Ok(Self::Ipv6),
            _ => Err(AppError::Socks5("invalid atyp".into()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_addrs_ipv4() {
        // IPv4: 8.8.8.8:53 (Google DNS)
        let buf = &[0x08, 0x08, 0x08, 0x08, 0x00, 0x35];
        assert!(Atyp::IpV4.parse_addrs(buf).is_ok());
    }

    #[test]
    fn test_parse_addrs_domain_name() {
        let buf = &[
            0x0a, // domain length: 10 bytes
            b'g', b'o', b'o', b'g', b'l', b'e', b'.', b'c', b'o', b'm', // google.com
            0x01, 0xbb // port: 443
        ];
        assert!(Atyp::DomainName.parse_addrs(buf).is_ok());
    }

    #[test]
    fn test_parse_addrs_ipv6() {
        // IPv6: 2001:4860:4860::8888:53 (Google DNS)
        let buf = &[
            0x20, 0x01, 0x48, 0x60, 0x48, 0x60, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x88, 
            0x00, 0x35
        ];
        assert!(Atyp::Ipv6.parse_addrs(buf).is_ok());
    }
}