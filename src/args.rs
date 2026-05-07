use std::{env::args, net::Ipv4Addr, str::FromStr};

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Arg {
    Host(Ipv4Addr),
    Port(u16),
    Auth((String, String))
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
        iter.into_iter()
            .skip(1)
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<String>>()
            .chunks(2)
            .map(|chunk| {
                let [key, value] = chunk else { 
                    return Err(AppError::Arguments(format!("invalid argument format {chunk:?} (expected --key value)"))); 
                };
                if !key.starts_with("--") { return Err(AppError::Arguments(format!("invalid argument syntax: {key} (must start with --)"))); }
                Self::from_string(key, value)
            })
            .collect()
    }

    fn from_string(str: &str, value: &str) -> Result<Self, AppError> {
        match str {
            "--host" => Ipv4Addr::from_str(value)
                .map(Self::Host)
                .map_err(|_| AppError::Arguments(format!("invalid host: {value}"))),
            "--port" => value.parse()
                .map(Self::Port)
                .map_err(|_| AppError::Arguments(format!("invalid port: {value}"))),
            "--auth" => value
                .split_once(":")
                .map(|(user, pass)| Self::Auth((user.to_string(), pass.to_string())))
                .ok_or_else(|| AppError::Arguments(format!("invalid auth format: {value} (expected username:password)"))),
            _ => Err(AppError::Arguments(format!("unknown argument {str}")))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_args() {
        let args = vec!["program", "--host", "127.0.0.1", "--port", "3000"];
        let result = Arg::from_args(args).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Arg::Host(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(result[1], Arg::Port(3000));
    }

    #[test]
    fn test_missing_value() {
        let args = vec!["program", "--port"];
        assert!(Arg::from_args(args).is_err());
    }

    #[test]
    fn test_invalid_host() {
        let args = vec!["program", "--host", "256.256.256.256"];
        assert!(Arg::from_args(args).is_err());
    }

    #[test]
    fn test_invalid_port() {
        let args = vec!["program", "--port", "foo"];
        assert!(Arg::from_args(args).is_err());
    }

    #[test]
    fn test_auth() {
        let args = vec!["program", "--auth", "user:pass"];
        assert!(Arg::from_args(args).is_ok());
    }
}