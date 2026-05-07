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
    let config = Socks5Config::new()?;

    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await
        .map_err(|_| AppError::Socks5(format!("port {} is busy or cannot be used", config.port)))?;

    let mut msg_output = format!("socks5 {}:{} started", config.host, config.port);
    if let Some((user, pass)) = &config.auth { 
        msg_output.push_str(format!(" | auth user: {}, pass: {}", user, pass).as_str());
    }
    println!("{}", msg_output.cyan());

    loop {
        let (stream, _) = listener.accept().await?;
        let config = config.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_request(config, stream).await {
                eprintln!("{e}");
            }
        });
    }
}

async fn handle_request(config: Socks5Config, stream: TcpStream) -> Result<(), AppError> {
    Socks5::new(config, stream).serve().await
}