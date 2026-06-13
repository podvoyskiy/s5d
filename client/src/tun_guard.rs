use std::{io::Read, net::Ipv4Addr};
use rtnetlink::{Handle, RouteMessageBuilder, new_connection, packet_route::{route::RouteMessage, rule::RuleAction}};

use s5d_lib::colorize::Colorize;
use tun::{AbstractDevice, Device};
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
        
        let tun_index = dev.tun_index().map_err(|e| AppError::ModeTun(format!("{e}")))?;

        let (handle, route) = Self::setup_routes(tun_index as u32).await?;
        
        Ok(Self { dev, handle, route })
    }

    pub async fn setup_routes(tun_index: u32) -> Result<(Handle, RouteMessage), AppError> {
        let (connection, handle, _) = new_connection()?;
        tokio::spawn(connection);
        
        // adding the default route to table 100 via TUN
        let destination = Ipv4Addr::UNSPECIFIED;
        let prefix_len = 0;
        let route = RouteMessageBuilder::<Ipv4Addr>::new()
            .destination_prefix(destination, prefix_len)
            .output_interface(tun_index)
            .table_id(TABLE_ID)
            .build();

        match handle.route().add(route.clone()).execute().await {
            Ok(_) => {},
            Err(e) => {
                if e.to_string().contains("File exists") {
                    println!("{}", "route already exists, continue...".yellow());
                } else {
                    return Err(AppError::ModeTun(format!("failed to create ip route: {e}")));
                }
            },
        }

        // adding a rule that all traffic goes to table 100
        //TODO правило не удаляется автоматически. надо чистить
        handle
            .rule()
            .add()
            .v4()
            .output_interface(TUN_NAME.to_string())
            .action(RuleAction::ToTable)
            .table_id(TABLE_ID)
            .priority(1000)
            .execute()
            .await
            .map_err(|e| AppError::ModeTun(format!("failed to create ip route: {e}")))?;

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