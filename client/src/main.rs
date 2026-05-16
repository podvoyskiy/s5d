mod prelude;
mod args;
mod mode;
mod socks5;
mod http;

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

    info!(mode = ?config.mode, proxy = ?config.proxy, "socks5 client started");

    let mut stream = TcpStream::connect(config.proxy).await.map_err(|_| AppError::TargetUnreachable)?;

    //handshake
    let mut methods = vec![consts::auth::NO_AUTH];
    if config.auth.is_some() { methods.push(consts::auth::AUTH); }

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
    if buf[1] == consts::auth::AUTH {
        let (username, password) = config.auth.unwrap();
        let mut auth = Vec::with_capacity(1 + 1 + username.len() + 1 + password.len());
        auth.push(consts::auth::VERSION);
        auth.push(username.len() as u8);
        auth.extend_from_slice(username.as_bytes());
        auth.push(password.len() as u8);
        auth.extend_from_slice(password.as_bytes());

        stream.write_all(&auth).await?;

        let mut buf = [0; 2];
        stream.read_exact(&mut buf).await?;
        trace!(?buf, "auth");

        if buf[0] != consts::auth::VERSION || buf[1] != consts::reply::SUCCESS { 
            return Err(AppError::AuthFailed); 
        }
    }

    //connect
    let mut connect = vec![consts::SOCKS_VERSION, consts::connect::CMD, consts::RSV];
    connect.extend_from_slice(&config.target.as_ref().unwrap().to_bytes());

    stream.write_all(&connect).await?;

    let mut buf = [0; 10];
    stream.read_exact(&mut buf).await?;
    trace!(?buf, "connect");

    if buf[0] != consts::SOCKS_VERSION || buf[1] != consts::reply::SUCCESS { 
        return Err(AppError::ConnectFailed); 
    }

    Ok(())
}