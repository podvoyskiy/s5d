#![warn(clippy::all)]

mod args;
mod errors;
mod colorize;
mod socks5;
mod prelude;

use prelude::*;
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::fmt;
use crate::socks5::{config::Socks5Config, connection::{Socks5}};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let config = Socks5Config::new()?;

    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await
        .map_err(|_| AppError::Socks5(format!("port {} is busy or cannot be used", config.port)))?;

    info!("socks5 {}:{} started", config.host, config.port);
    if let Some((user, pass)) = &config.auth { 
        info!("auth enabled: {}:{}", user, pass);
    }

    loop {
        let config = config.clone();
        let (stream, client_addr) = listener.accept().await?;
        info!(%client_addr, "new connection");
        tokio::spawn(async move {
            let mut socks5 = Socks5::new(config, stream);
            if let Err(error) = socks5.serve().await {
                error!(%error, %client_addr);
            }
            info!(%client_addr, "connection closed");
        });
    }
}