#![warn(clippy::all)]

mod args;
mod errors;
mod colorize;
mod socks5;
mod prelude;

use prelude::*;
use tokio::net::{TcpListener, TcpStream};
use crate::socks5::{config::Socks5Config, connection::{Socks5}};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let socks5_config = Socks5Config::new()?;

    let listener = TcpListener::bind(format!("{}:{}", socks5_config.host, socks5_config.port)).await
        .map_err(|_| AppError::Socks5(format!("port {} is busy or cannot be used", socks5_config.port)))?;

    println!("{}", format!("socks5 {}:{} started", socks5_config.host, socks5_config.port).cyan());

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_request(stream).await {
                eprintln!("{e}");
            }
        });
    }
}

async fn handle_request(stream: TcpStream) -> Result<(), AppError> {
    Socks5::new(stream).serve().await
}