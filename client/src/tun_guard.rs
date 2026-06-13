use std::{io::Read, process::Command};

use s5d_lib::colorize::Colorize;
use tun::Device;

use crate::prelude::*;

const TUN_NAME: &str = "tun0";
const TABLE_ID: u16 = 100;

pub struct TunGuard {
    dev: Device
}

impl TunGuard {
    pub fn new() -> Result<Self, AppError> {
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
        
        Ok(Self { dev })
    }

    pub fn setup_routes() -> Result<(), AppError> {
        // adding the default route to table 100 via TUN
        let status = Command::new("ip")
            .args(["route", "add", "default", "dev", TUN_NAME, "table", &TABLE_ID.to_string()])
            .status()
            .map_err(|e| AppError::ModeTun(format!("ip route failed: {e}")))?;

        if !status.success() { eprintln!("{}", format!("default route in table {TABLE_ID} might already exist").yellow()) }

        // adding a rule that all traffic goes to table 100
        let status = Command::new("ip")
            .args(["rule", "add", "from", "all", "lookup", &TABLE_ID.to_string(), "priority", "1000"])
            .status()
            .map_err(|e| AppError::ModeTun(format!("ip rule failed: {e}")))?;

        if !status.success() { eprintln!("{}", "rule with priority 1000 might already exist".yellow()) }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), AppError> {
        let mut buf = [0; 4096];
        loop {
            let amount = self.dev.read(&mut buf)?;
            println!("{:?}", &buf[0..amount]);
        }
    }

    fn cleanup_routes() {
        let _ = Command::new("ip")
            .args(["route", "del", "default", "dev", TUN_NAME, "table", &TABLE_ID.to_string()])
            .status();

        let _ = Command::new("ip")
            .args(["rule", "del", "priority", "1000"])
            .status();
    }
}

impl Drop for TunGuard {
    fn drop(&mut self) {
        Self::cleanup_routes();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}