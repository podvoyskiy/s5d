use std::env::args;
use std::net::SocketAddr;
use std::str::FromStr;

use crate::http::{Http, Method};
use crate::prelude::*;

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
        Self::from_args(args())
    }
    
    fn from_args<I, S>(iter: I) -> Result<Self, AppError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut config = Self::default();
        for (key, value)  in utils::collect_args(iter)? { config.set_param(&key, &value)?; }
        Ok(config)
    }

    fn set_param(&mut self, key: &str, value: &str) -> Result<(), AppError> {
        match key {
            "--mode" => {
                self.mode = Mode::try_from(value)?;
                Ok(())
            }
            "--proxy" => {
                SocketAddr::from_str(value)
                    .map(|addr| self.proxy = addr)
                    .map_err(|_| AppError::Arguments("invalid proxy addr".into()))?;
                Ok(())
            }
            "--auth" => {
                value
                    .split_once(":")
                    .map(|(user, pass)| self.auth = Some((user.to_string(), pass.to_string())))
                    .ok_or_else(|| AppError::Arguments(format!("invalid auth format: {value} (expected username:password)")))?;
                Ok(())
            }
            "--target" => {
                Atyp::from_str(value)
                    .map(|atyp| self.target = Some(atyp))
                    .map_err(|_| AppError::Arguments(format!("invalid target: {value}")))?;
                Ok(())
            }
            "--method" => {
                let method = value.parse::<Method>()?;
                match &mut self.http {
                    Some(http) => http.method = method,
                    None => self.http = Some(Http { method, data: None, headers: None }),
                }
                Ok(())
            },
            _ => Err(AppError::Arguments(format!("unknown argument {key}")))
        }
    }

    pub fn validate(&mut self) -> Result<(), AppError> {
        if self.mode == Mode::Cli {
            if self.target.is_none() { return Err(AppError::Arguments("missed param --target".into())); }
            if self.http.is_none() { self.http = Some(Http::default()) }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod test {
use super::*;

    #[test]
    fn test_valid_args() {
        let args = vec!["program", "--mode", "cli", "--proxy", "127.0.0.1:1080"];
        let config = Socks5Config::from_args(args).unwrap();
        assert_eq!(config.mode, Mode::Cli);
        assert_eq!(config.proxy, SocketAddr::from(([127, 0, 0, 1], 1080)));
    }
}