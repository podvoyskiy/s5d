use std::net::SocketAddr;

use crate::prelude::*;
use crate::args::Arg;

#[derive(Debug, Clone)]
pub struct Socks5Config {
    pub mode: Mode,
    pub server: SocketAddr,
    pub auth: Option<(String, String)>,
    pub target: Option<String>
}

impl Default for Socks5Config  {
    fn default() -> Self {
        Self { mode: Mode::Cli, server: SocketAddr::from(([127, 0, 0, 1], 1080)), auth: None, target: None }
    }
}

impl Socks5Config {
    pub fn new() -> Result<Self, AppError> {
        let mut config = Self::default();

        for arg in Arg::init()? {
            match arg {
                Arg::Mode(mode) => config.mode = mode,
                Arg::Server(addr) => config.server = addr,
                Arg::Auth(auth) => config.auth = Some(auth),
                Arg::Target(target) => config.target = Some(target),
            }
        }

        if config.mode == Mode::Cli && config.target.is_none() { return Err(AppError::Arguments("missed param --target".into())); }
        
        Ok(config)
    }
}
