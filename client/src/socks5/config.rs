use std::net::SocketAddr;

use crate::prelude::*;
use crate::args::Arg;

#[derive(Debug, Clone)]
pub struct Socks5Config {
    pub mode: Mode,
    pub server: SocketAddr,
    pub auth: Option<(String, String)>
}

impl Default for Socks5Config  {
    fn default() -> Self {
        Self { mode: Mode::Cli, server: SocketAddr::from(([127, 0, 0, 1], 1080)), auth: None }
    }
}

impl Socks5Config {
    pub fn new() -> Result<Self, AppError> {
        let mut socks5 = Self::default();

        for arg in Arg::init()? {
            match arg {
                Arg::Mode(mode) => socks5.mode = mode,
                Arg::Server(addr) => socks5.server = addr,
                Arg::Auth(auth) => socks5.auth = Some(auth),
            }
        }
        
        Ok(socks5)
    }
}
