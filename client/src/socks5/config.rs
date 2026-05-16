use std::net::SocketAddr;

use crate::http::Http;
use crate::prelude::*;
use crate::args::Arg;

#[derive(Debug)]
pub struct Socks5Config {
    pub mode: Mode,
    pub proxy: SocketAddr,
    pub auth: Option<(String, String)>,
    //for cli mode
    pub target: Option<Atyp>,
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

        Ok(config)
    }

    pub fn validate(&mut self) -> Result<(), AppError> {
        if self.mode == Mode::Cli {
            if self.target.is_none() { return Err(AppError::Arguments("missed param --target".into())); }
            if self.http.is_none() { self.http = Some(Http::default()) }
        }
        
        Ok(())
    }
}
