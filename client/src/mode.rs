use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Cli,
    Proxy,
    Tun,
}

impl TryFrom<&str> for Mode {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cli" => Ok(Self::Cli),
            "proxy" => Ok(Self::Proxy),
            "tun" => Ok(Self::Tun),
            _ => Err(AppError::Socks5("invalid mode".into()))
        }
    }
}