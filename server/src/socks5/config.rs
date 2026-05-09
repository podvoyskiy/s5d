use std::net::Ipv4Addr;

use crate::prelude::*;
use crate::args::Arg;

#[derive(Debug, Clone)]
pub struct Socks5Config {
    pub host: Ipv4Addr,
    pub port: u16,
    pub auth: Option<(String, String)>
}

impl Default for Socks5Config  {
    fn default() -> Self {
        Self { host: Ipv4Addr::LOCALHOST, port: 1080, auth: None }
    }
}

impl Socks5Config {
    pub fn new() -> Result<Self, AppError> {
        let mut socks5 = Self::default();

        for arg in Arg::init()? {
            match arg {
                Arg::Host(ip) => socks5.host = ip,
                Arg::Port(port) => socks5.port = port,
                Arg::Auth(auth) => socks5.auth = Some(auth),
            }
        }
        if socks5.port == 0 { return Err(AppError::Arguments("port cannot be 0".into())); }
        
        Ok(socks5)
    }
}
