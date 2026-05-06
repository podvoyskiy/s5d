use std::net::SocketAddr;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use crate::{prelude::*, socks5::{addr, atyp::Atyp, consts}};

#[derive(Debug, PartialEq)]
enum Socks5State {
    Handshake,
    Connect,
    Tunneling
}

pub struct Socks5 {
    state: Socks5State,
    client: Option<TcpStream>,
    target: Option<TcpStream>,
}

impl Socks5 {
    pub fn new(client: TcpStream) -> Self {
        Self { state: Socks5State::Handshake, client: Some(client), target: None }
    }

    pub async fn serve(&mut self) -> Result<(), AppError> {
        let mut buf = [0; 4096];

        loop {
            match self.client.as_mut().unwrap().read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    match self.state {
                        Socks5State::Handshake => self.handshake(&buf[..n]).await?,
                        Socks5State::Connect => {
                            let target_addrs = self.handle_connect(&buf[..n])?;
                            self.connect_to_target(target_addrs).await?;
                        },
                        Socks5State::Tunneling => {
                            self.target.as_mut().unwrap().write_all(&buf[..n]).await?;
                            
                            let (mut client_r, mut client_w) = self.client.take().unwrap().into_split();
                            let (mut target_r, mut target_w) = self.target.take().unwrap().into_split();

                            let client_to_target = tokio::spawn(async move {
                                let _ = tokio::io::copy(&mut client_r, &mut target_w).await;
                            });

                            let target_to_client = tokio::spawn(async move {
                                let _ = tokio::io::copy(&mut target_r, &mut client_w).await;
                            });

                            let _ = tokio::join!(client_to_target, target_to_client);
                            println!("{}", "connection closed".orange());

                            break;
                        },
                    }
                },
                Err(e) => return Err(AppError::Socks5(format!("read error: {e}"))),
            }
        }

        Ok(())
    }

    async fn handshake(&mut self, buf: &[u8]) -> Result<(), AppError> {
        if buf.first() != Some(&consts::VERSION) { return Err(AppError::HandshakeFailed); }
        if buf.get(2) != Some(&consts::NO_AUTH) {
            self.client.as_mut().unwrap().write_all(&[consts::VERSION, consts::reply::NO_ACCEPTABLE_METHOD]).await?;
            return Err(AppError::HandshakeFailed);
        }
        self.state = Socks5State::Connect;
        self.client.as_mut().unwrap().write_all(&[consts::VERSION, consts::reply::SUCCESS]).await?;
        Ok(())
    }

    fn handle_connect(&mut self, buf: &[u8]) -> Result<Vec<SocketAddr>, AppError> {
        if buf.len() < 4 || buf[0] != consts::VERSION || buf[1] != consts::CMD_CONNECT { return Err(AppError::ConnectFailed); }
        
        let atyp = Atyp::try_from(buf[3])?;
        let data = &buf.get(4..).ok_or(AppError::ConnectFailed)?;
        atyp.parse_addrs(data)
    }

    async fn connect_to_target(&mut self, target_addrs: Vec<SocketAddr>) -> Result<(), AppError> {
        for addr in target_addrs {
            let stream = TcpStream::connect(addr).await;
            if stream.is_err() { continue; }

            self.target = Some(stream.unwrap());
            println!("{} {}", "successfully connected to".green(), addr.to_string().green());
            
            let mut response = Vec::with_capacity(10);
            response.extend_from_slice(&[consts::VERSION, consts::reply::SUCCESS, consts::reply::RSV]);
            response.extend(&[if addr.is_ipv4() { Atyp::IpV4 as u8 } else { Atyp::Ipv6 as u8 }]);
            response.extend(addr::to_bytes(self.target.as_ref().unwrap())?);

            self.state = Socks5State::Tunneling;
            self.client.as_mut().unwrap().write_all(&response).await?;
            return Ok(());
        }

        eprintln!("{}", "failed connected to target".red());

        let mut response = Vec::with_capacity(10);
        response.extend_from_slice(&[consts::VERSION, consts::reply::FAILURE, consts::reply::RSV, Atyp::IpV4 as u8]);
        response.extend(consts::reply::BND_ADDR);
        response.extend(consts::reply::BND_PORT);

        self.client.as_mut().unwrap().write_all(&response).await?;
        Err(AppError::TargetUnreachable)
    }
}