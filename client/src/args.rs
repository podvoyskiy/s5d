use std::{env::args, str::FromStr};
use std::net::SocketAddr;

use crate::http::method::HttpMethod;
use crate::{mode::Mode, prelude::*};

#[derive(Debug, PartialEq)]
pub enum Arg {
    Mode(Mode),
    Proxy(SocketAddr),
    Auth((String, String)),

    Target(Atyp),
    Method(HttpMethod)
}

impl Arg {
    pub fn init() -> Result<Vec<Self>, AppError> {
        Self::from_args(args())
    }

    fn from_args<I, S>(iter: I) -> Result<Vec<Self>, AppError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        utils::collect_args(iter)?
            .into_iter()
            .map(|(key, value)| Self::from_string(&key, &value))
            .collect()
    }

    fn from_string(key: &str, value: &str) -> Result<Self, AppError> {
        match key {
            "--mode"   => Ok(Self::Mode(Mode::try_from(value)?)),
            "--proxy" => SocketAddr::from_str(value)
                .map(Self::Proxy)
                .map_err(|_| AppError::Arguments("invalid proxy addr".into())),
            "--auth" => value
                .split_once(":")
                .map(|(user, pass)| Self::Auth((user.to_string(), pass.to_string())))
                .ok_or_else(|| AppError::Arguments(format!("invalid auth format: {value} (expected username:password)"))),

            "--target" => {
                Atyp::from_str(value)
                    .map(Self::Target)
                    .map_err(|_| AppError::Arguments(format!("invalid target: {value}")))
            },
            "--method" => value.parse::<HttpMethod>().map(Self::Method),
            _  => Err(AppError::Arguments(format!("unknown argument {key}")))
        }
    }
}

#[cfg(test)]
mod test {
use super::*;

    #[test]
    fn test_valid_args() {
        let args = vec!["program", "--mode", "cli", "--proxy", "127.0.0.1:1080"];
        let result = Arg::from_args(args).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Arg::Mode(Mode::Cli));
        assert_eq!(result[1], Arg::Proxy(SocketAddr::from(([127, 0, 0, 1], 1080))));
    }
}