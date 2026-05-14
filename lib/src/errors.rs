use std::{fmt::{self, Debug, Display}, io};

use crate::colorize::Colorize;

pub enum AppError {
    Io(io::Error),
    Arguments(String),
    Socks5(String),
    HandshakeFailed,
    AuthFailed,
    ConnectFailed,
    InvalidAtyp,
    InvalidDomain,
    InvalidIpv4,
    InvalidIpv6,
    TargetUnreachable,
}

impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            AppError::Io(err) => format!("I/O error | {err}").red(),
            AppError::Arguments(err) => format!("arguments error: {err}").red(),
            AppError::Socks5(err) => format!("socks5 error: {err}").red(),
            AppError::HandshakeFailed => "socks5 handshake failed".red(),
            AppError::AuthFailed => "socks5 auth failed".red(),
            AppError::ConnectFailed => "socks5 connect failed".red(),
            AppError::InvalidAtyp => "socks5 invalid atyp".red(),
            AppError::InvalidDomain => "socks5 invalid domain".red(),
            AppError::InvalidIpv4 => "socks5 invalid ipv4".red(),
            AppError::InvalidIpv6 => "socks5 invalid ipv6".red(),
            AppError::TargetUnreachable => "socks5 target unreachable".red(),
        };
        write!(f, "{message}")
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError::Io(value)
    }
}