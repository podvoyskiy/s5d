use std::{io::Read, net::Ipv4Addr};
use rtnetlink::{Handle, RouteMessageBuilder, new_connection, packet_route::route::RouteMessage};

use tun::Device;
use crate::prelude::*;

const TUN_NAME: &str = "tun0";
const TABLE_ID: u32 = 100;

pub struct TunGuard {
    dev: Device,
    handle: Handle,
    route: RouteMessage,
}

impl TunGuard {
    pub async fn new() -> Result<Self, AppError> {
        let mut config = tun::Configuration::default();
            config
                .address((10, 0, 0, 9))
                .netmask((255, 255, 255, 0))
                .destination((10, 0, 0, 1))
                .up();
        
        #[cfg(target_os = "linux")]
        config.platform_config(|config| { config.ensure_root_privileges(true); });

        let dev = tun::create(&config)
            .map_err(|e| AppError::ModeTun(format!("failed to create tun interface: {e}")))?;

        let (handle, route) = Self::setup_routes().await?;
        
        Ok(Self { dev, handle, route })
    }

    pub async fn setup_routes() -> Result<(Handle, RouteMessage), AppError> {
        let (connection, handle, _) = new_connection()?;
        tokio::spawn(connection);
        
        // adding the default route to table 100 via TUN
        let gateway = Ipv4Addr::new(192, 168, 1, 1);
        let destination = Ipv4Addr::UNSPECIFIED;
        let prefix_len = 0;
        let route = RouteMessageBuilder::<Ipv4Addr>::new()
            .destination_prefix(destination, prefix_len)
            .gateway(gateway)
            .table_id(TABLE_ID)
            .build();

        handle.route().add(route.clone()).execute().await
            .map_err(|e| AppError::ModeTun(format!("failed to create ip route: {e}")))?;

        // // adding a rule that all traffic goes to table 100
        // let status = Command::new("ip")
        //     .args(["rule", "add", "from", "all", "lookup", &TABLE_ID.to_string(), "priority", "1000"])
        //     .status()
        //     .map_err(|e| AppError::ModeTun(format!("ip rule failed: {e}")))?;

        // if !status.success() { eprintln!("{}", "rule with priority 1000 might already exist".yellow()) }

        Ok((handle, route))
    }

    pub fn run(&mut self) -> Result<(), AppError> {
        let mut buf = [0; 4096];
        loop {
            let amount = self.dev.read(&mut buf)?;
            println!("{:?}", &buf[0..amount]);
        }
    }

    async fn cleanup_routes(&mut self) {
        let _ = self.handle.route().del(self.route.clone()).execute().await;

        //handle.rule().del().priority(1000).execute().await?;
    }
}

impl Drop for TunGuard {
    fn drop(&mut self) {
        tokio::runtime::Handle::current().block_on(async {
            let _ = self.cleanup_routes();
        });
    }
}