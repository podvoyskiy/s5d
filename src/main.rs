mod args;
mod errors;
mod colorize;
mod socks5;
mod prelude;

use std::{net::{TcpListener, TcpStream}, thread};

use prelude::*;
use crate::socks5::{config::Socks5Config, connection::{Socks5}};

fn main() -> Result<(), AppError> {
    let socks5_config = Socks5Config::new()?;

    let listener = TcpListener::bind(format!("{}:{}", socks5_config.host, socks5_config.port))
        .map_err(|_| AppError::Socks5(format!("port {} is busy or cannot be used", socks5_config.port)))?;

    println!("{}", format!("socks5 {}:{} started", socks5_config.host, socks5_config.port).cyan());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_request(stream) {
                        eprintln!("{e}");
                    }
                });
            },
            Err(error) => eprintln!("{} {}", "accept failed:".red(), error.to_string().red()),
        }
    } 

    Ok(())
}

fn handle_request(stream: TcpStream) -> Result<(), AppError> {
    Socks5::new(stream).serve()
}