#![warn(clippy::pedantic)]

mod prelude;
mod mode;
mod socks5;
mod http;

use prelude::*;
use tokio::net::TcpStream;
use tracing::Level;
use tracing_subscriber::fmt;

use crate::socks5::{config::Socks5Config, session::Socks5Session};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let mut config = Socks5Config::new()?;
    config.validate()?;

    debug!(config = ?config, "socks5 client started");

    if config.mode != Mode::Cli {
        return Err(AppError::Socks5(format!("mode {:?} not yet implemented", config.mode)));
    }

    let stream = TcpStream::connect(config.proxy).await.map_err(|_| AppError::TargetUnreachable)?;

    let mut session = Socks5Session::new(config, stream);
    if session.handshake().await? == consts::auth::AUTH {
        session.auth().await?;
    }
    session.connect().await?;
    session.send().await
}