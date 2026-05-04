use std::net::Ipv4Addr;

use crate::prelude::*;
use crate::args::Arg;

#[derive(Debug)]
pub struct Socks5Config {
    pub host: Ipv4Addr,
    pub port: u16
}

impl Default for Socks5Config  {
    fn default() -> Self {
        Self { host: Ipv4Addr::LOCALHOST, port: 1080 }
    }
}

impl Socks5Config {
    pub fn new() -> Result<Self, AppError> {
        let mut socks5 = Self::default();

        for arg in Arg::init()? {
            match arg {
                Arg::Host(ip) => socks5.host = ip,
                Arg::Port(value) => socks5.port = value,
            }
        }
        if socks5.port == 0 { return Err(AppError::Arguments("port cannot be 0".into())); }
        
        Ok(socks5)
    }
}
