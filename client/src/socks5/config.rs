use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use crate::http::{Http, Method};
use crate::prelude::*;

#[derive(Debug)]
pub struct Socks5Config {
    pub mode: Mode,
    pub proxy: SocketAddr,
    pub auth: Option<(String, String)>,
    pub target: Option<Atyp>,
    pub http: Http,
    pub use_tls: bool,
}

impl Default for Socks5Config  {
    fn default() -> Self {
        Self { mode: Mode::Cli, proxy: SocketAddr::from(([127, 0, 0, 1], 1080)), auth: None, target: None, http: Http::default(), use_tls: false }
    }
}

impl Config for Socks5Config {
    fn set_param(&mut self, key: &str, value: &str) -> Result<(), AppError> {
        match key {
            "--mode" => {
                self.mode = Mode::try_from(value)?;
                Ok(())
            }
            "--proxy" => {
                self.proxy = value.parse()
                    .map_err(|_| AppError::Arguments("invalid proxy addr".into()))?;
                Ok(())
            }
            "--auth" => {
                value
                    .split_once(':')
                    .map(|(user, pass)| self.auth = Some((user.to_string(), pass.to_string())))
                    .ok_or_else(|| AppError::Arguments(format!("invalid auth format: {value} (expected username:password)")))?;
                Ok(())
            }
            "--target" => {
                self.http.path = utils::extract_path(value);
                self.use_tls = value.starts_with("https://");
                Atyp::from_str(value)
                    .map(|atyp| self.target = Some(atyp))
                    .map_err(|_| AppError::Arguments(format!("invalid target: {value}")))?;
                Ok(())
            }
            "--method" => {
                self.http.method = value.parse::<Method>()?;
                Ok(())
            },
            "--data" => {
                self.http.data = Some(value.to_string());
                if self.http.method == Method::GET { self.http.method = Method::POST; }
                Ok(())
            },
             "--headers" => {
                let header = value
                    .split_once(':')
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .ok_or_else(|| AppError::Arguments(format!("invalid headers format: {value} (expected key:value)")))?;
                if let Some(headers) = &mut self.http.headers {
                    headers.push(header);
                } else {
                    self.http.headers = Some(vec![header]);
                }
                Ok(())
            }
            _ => Err(AppError::Arguments(format!("unknown argument {key}")))
        }
    }
    
    fn validate(&mut self) -> Result<(), AppError> {
        if self.mode == Mode::Cli && self.target.is_none() {
            return Err(AppError::Arguments("missed param --target".into()));
        }
        if self.use_tls && self.target.as_ref().unwrap().host_str().parse::<IpAddr>().is_ok() {
            return Err(AppError::Arguments("invalid target: https requires domain name, not IP".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

use super::*;

    #[test]
    fn test_valid_args() {
        let args = vec!["program", "--mode", "cli", "--proxy", "127.0.0.1:1080", "--target", "https://example.com:8443"];
        let mut config = Socks5Config::from_args(args).unwrap();
        assert!(config.validate().is_ok());
        assert_eq!(config.mode, Mode::Cli);
        assert_eq!(config.proxy, SocketAddr::from(([127, 0, 0, 1], 1080)));
        assert_eq!(config.use_tls, true);
    }

    #[test]
    fn test_valid_args_with_http_headers() {
        let args = vec![
            "program", 
            "--mode", "cli", 
            "--proxy", "127.0.0.1:1080", 
            "--target", "https://example.com",
            "--headers", "Content-Type:application/json",
            "--headers", "Authorization:Bearer qwerty123",
        ];
        let mut config = Socks5Config::from_args(args).unwrap();
        assert!(config.validate().is_ok());

        let headers = config.http.headers.unwrap();
        assert_eq!(headers.len(), 2);
        assert_eq!(headers.first().unwrap(), &("Content-Type".to_string(), "application/json".to_string()));
        assert_eq!(headers.last().unwrap(), &("Authorization".to_string(), "Bearer qwerty123".to_string()));
    }

    #[test]
    fn test_https_with_ip() {
        let args = vec!["program", "--mode", "cli", "--proxy", "127.0.0.1:1080", "--target", "https://34.234.10.121/get"];
        let mut config = Socks5Config::from_args(args).unwrap();
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("https requires domain name"));
    }
}