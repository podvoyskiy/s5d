use std::{io::{Read, Write}, net::{SocketAddr, TcpStream}, thread::{self, JoinHandle}};

use crate::{prelude::*, socks5::{addr, atyp::Atyp, consts}};

#[derive(Debug, PartialEq)]
enum Socks5State {
    Handshake,
    Connect,
    Tunneling
}

pub struct Socks5 {
    state: Socks5State,
    target_addrs: Vec<SocketAddr>,
    client: TcpStream,
    target: Option<TcpStream>,
}

impl Socks5 {
    pub fn new(client: TcpStream) -> Self {
        Self { state: Socks5State::Handshake, target_addrs: Vec::new(), client, target: None }
    }

    pub fn serve(&mut self) -> Result<(), AppError> {
        let mut buf = [0; 4096];

        loop {
            match self.client.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    match self.state {
                        Socks5State::Handshake => self.handshake(&buf[..n])?,
                        Socks5State::Connect => {
                            self.handle_connect(&buf[..n])?;
                            self.connect_to_target()?;
                        },
                        Socks5State::Tunneling => {
                            self.target.as_mut().unwrap().write_all(&buf[..n])?;

                            let rx_thread = self.pipe_to_client()?;
                            let tx_thread = self.pipe_to_target()?;
                            rx_thread.join().map_err(|_| AppError::Socks5("thread join".into()))?;
                            tx_thread.join().map_err(|_| AppError::Socks5("thread join".into()))?;

                            break;
                        },
                    }
                },
                Err(e) => return Err(AppError::Socks5(format!("read error: {e}"))),
            }
        }

        Ok(())
    }

    fn handshake(&mut self, buf: &[u8]) -> Result<(), AppError> {
        if buf.first() != Some(&consts::VERSION) { return Err(AppError::HandshakeFailed); }
        if buf.get(2) != Some(&consts::NO_AUTH) {
            self.client.write_all(&[consts::VERSION, consts::reply::NO_ACCEPTABLE_METHOD])?;
            return Err(AppError::HandshakeFailed);
        }
        self.state = Socks5State::Connect;
        self.client.write_all(&[consts::VERSION, consts::reply::SUCCESS])?;
        Ok(())
    }

    fn handle_connect(&mut self, buf: &[u8]) -> Result<(), AppError> {
        if buf.len() < 4 || buf[0] != consts::VERSION || buf[1] != consts::CMD_CONNECT { return Err(AppError::ConnectFailed); }
        
        let atyp = Atyp::try_from(buf[3])?;
        let data = &buf.get(4..).ok_or(AppError::ConnectFailed)?;
        self.target_addrs = atyp.parse_addrs(data)?;
        Ok(())
    }

    fn connect_to_target(&mut self) -> Result<(), AppError> {
        let mut response = Vec::with_capacity(10);

        for addr in &self.target_addrs {
            let stream = TcpStream::connect(addr);
            if stream.is_err() { continue; }

            self.target = Some(stream.unwrap());
            println!("{} {}", "successfully connected to".green(), addr.to_string().green());

            response.extend_from_slice(&[consts::VERSION, consts::reply::SUCCESS, consts::reply::RSV]);
            response.extend(&[if addr.is_ipv4() { Atyp::IpV4 as u8 } else { Atyp::Ipv6 as u8 }]);
            response.extend(addr::to_bytes(self.target.as_ref().unwrap())?);

            self.state = Socks5State::Tunneling;
            self.client.write_all(&response)?;
            return Ok(());
        }

        eprintln!("{}", "failed connected to target".red());

        response.extend_from_slice(&[consts::VERSION, consts::reply::FAILURE, consts::reply::RSV, Atyp::IpV4 as u8]);
        response.extend(consts::reply::BND_ADDR);
        response.extend(consts::reply::BND_PORT);

        self.client.write_all(&response)?;
        Err(AppError::TargetUnreachable)
    }

    fn pipe_to_client(&mut self) -> Result<JoinHandle<()>, AppError> {
        let mut client = self.client.try_clone()?;
        let mut target = self.target.as_mut().unwrap().try_clone()?;

        Ok(thread::spawn(move || {
            let mut buf = [0; 4096];
            loop {
                match target.read(&mut buf) {
                    Ok(0) => {
                        //в случае http данный поток закрывается по таймауту через 60 сек.
                        println!("{}", "target disconnected".orange());
                        break;
                    },
                    Ok(n) => {
                        let _ = client.write_all(&buf[..n]);
                    },
                    Err(_) => break,
                }
            }
        }))
    }

    fn pipe_to_target(&mut self) -> Result<JoinHandle<()>, AppError> {
        let mut client = self.client.try_clone()?;
        let mut target = self.target.as_mut().unwrap().try_clone()?;

        Ok(thread::spawn(move || {
            let mut buf = [0; 4096];
            loop {
                match client.read(&mut buf) {
                    Ok(0) => {
                        println!("{}", "client disconnected".orange());
                        break;
                    },
                    Ok(n) => {
                        let _ = target.write_all(&buf[..n]);
                    },
                    Err(_) => break,
                }
            }
        }))
    }
}