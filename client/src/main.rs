#![warn(clippy::pedantic)]

mod prelude;
mod mode;
mod socks5;
mod http;

use prelude::*;
use tokio::net::TcpStream;
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;

use crate::socks5::{config::Socks5Config, session::Socks5Session};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    setup_tracing();

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

#[cfg(debug_assertions)]
fn setup_tracing() {
    fmt()
        .with_target(false)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("rustls=warn,s5d_client=trace,s5d_lib=trace"))
        )
        .init();
}

#[cfg(not(debug_assertions))]
fn setup_tracing() {
    fmt()
        .with_target(false)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("rustls=error,s5d_client=info"))
        )
        .init();
}