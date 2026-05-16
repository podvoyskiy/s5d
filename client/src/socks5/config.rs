use std::net::SocketAddr;

use crate::http::method::HttpMethod;
use crate::prelude::*;
use crate::args::Arg;

#[derive(Debug)]
pub struct Socks5Config {
    pub mode: Mode,
    pub proxy: SocketAddr,
    pub auth: Option<(String, String)>,

    pub target: Option<Atyp>,
    pub method: Option<HttpMethod>,
}

impl Default for Socks5Config  {
    fn default() -> Self {
        Self { mode: Mode::Cli, proxy: SocketAddr::from(([127, 0, 0, 1], 1080)), auth: None, target: None, method: None }
    }
}

impl Socks5Config {
    pub fn new() -> Result<Self, AppError> {
        let mut config = Self::default();

        for arg in Arg::init()? {
            match arg {
                Arg::Mode(mode) => config.mode = mode,
                Arg::Proxy(addr) => config.proxy = addr,
                Arg::Auth(auth) => config.auth = Some(auth),
                Arg::Target(target) => config.target = Some(target),
                Arg::Method(http_method) => config.method = Some(http_method),
            }
        }

        if config.mode == Mode::Cli {
            if config.target.is_none() { return Err(AppError::Arguments("missed param --target".into())); }
            if config.method.is_none() { config.method = Some(HttpMethod::GET) }
        }
        
        Ok(config)
    }
}
