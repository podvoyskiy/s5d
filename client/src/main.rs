mod prelude;
mod args;
mod mode;
mod socks5;

use prelude::*;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tracing::Level;
use tracing_subscriber::fmt;

use crate::socks5::config::Socks5Config;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let config = Socks5Config::new()?;

    info!(mode = ?config.mode, server = ?config.server, "socks5 client started");

    let mut stream = TcpStream::connect(config.server).await.map_err(|_| AppError::TargetUnreachable)?;

    //handshake
    let handshake = &[consts::SOCKS_VERSION, 0x01, consts::NO_AUTH];
    stream.write_all(handshake).await?;

    let mut buf = [0; 2];
    stream.read_exact(&mut buf).await?;
    trace!(?buf, "handshake");
    if buf[0] != consts::SOCKS_VERSION || buf[1] != consts::NO_AUTH { return Err(AppError::AuthFailed); }

    //connect TODO

    Ok(())
}