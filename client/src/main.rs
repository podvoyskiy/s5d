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
    let mut methods = vec![consts::NO_AUTH];
    if config.auth.is_some() { methods.push(consts::AUTH); }

    let mut handshake = Vec::with_capacity(2 + methods.len());
    handshake.push(consts::SOCKS_VERSION);
    handshake.push(methods.len() as u8);
    handshake.extend_from_slice(&methods);

    stream.write_all(&handshake).await?;

    let mut buf = [0; 2];
    stream.read_exact(&mut buf).await?;
    trace!(?buf, "handshake");
    if buf[0] != consts::SOCKS_VERSION || !methods.contains(&buf[1]) { 
        return Err(AppError::HandshakeFailed); 
    }

    //auth
    if buf[1] == consts::AUTH {
        let (username, password) = config.auth.unwrap();
        let mut buf = Vec::with_capacity(1 + 1 + username.len() + 1 + password.len());
        buf.push(consts::AUTH_VERSION);
        buf.push(username.len() as u8);
        buf.extend_from_slice(username.as_bytes());
        buf.push(password.len() as u8);
        buf.extend_from_slice(password.as_bytes());

        stream.write_all(&buf).await?;

        let mut buf = [0; 2];
        stream.read_exact(&mut buf).await?;
        trace!(?buf, "auth");

        if buf[0] != consts::AUTH_VERSION || buf[1] != consts::reply::SUCCESS { 
            return Err(AppError::AuthFailed); 
        }
    }
    
    //connect
    let connect = &[
        consts::NO_AUTH
    ];

    Ok(())
}