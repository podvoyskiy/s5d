use std::net::SocketAddr;

use crate::http::Http;
use crate::prelude::*;
use crate::args::Arg;

#[derive(Debug)]
pub struct Socks5Config {
    pub mode: Mode,
    pub proxy: SocketAddr,
    pub auth: Option<(String, String)>,

    pub target: Option<Atyp>, //TODO может тоже в http перенести
    pub http: Option<Http>,
}

impl Default for Socks5Config  {
    fn default() -> Self {
        Self { mode: Mode::Cli, proxy: SocketAddr::from(([127, 0, 0, 1], 1080)), auth: None, target: None, http: None }
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
                Arg::Method(method) => config.http = Some(Http { method, data: None, headers: None }),
            }
        }

        if config.mode == Mode::Cli {
            if config.target.is_none() { return Err(AppError::Arguments("missed param --target".into())); }
            if config.http.is_none() { config.http = Some(Http::default()) }
        }
        
        Ok(config)
    }
}
