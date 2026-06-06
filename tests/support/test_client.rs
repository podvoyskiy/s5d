#![allow(dead_code)]

use std::process::{Command, Child};
use std::thread;
use std::time::Duration;

pub struct TestClient {
    child: Child
}

impl TestClient {
    pub fn start(proxy: &str, target: &str) -> Self {
        let child = 
            Command::new("./../target/debug/s5d-client")
            .arg("--proxy").arg(proxy)
            .arg("--target").arg(target)
            .spawn().unwrap();

        thread::sleep(Duration::from_millis(200));

        Self { child }
    }

    pub fn run(proxy: &str, target: &str, auth: Option<(String, String)>, data: Option<&str>, headers: Option<&str>, xor: Option<u8>) -> String {
        let mut cmd = Command::new("./../target/debug/s5d-client");
        cmd
            .arg("--proxy").arg(proxy)
            .arg("--target").arg(target);

        if let Some((user, pass)) = &auth {
            cmd.arg("--auth").arg(format!("{user}:{pass}"));
        }

        if let Some(data) = &data {
            cmd.arg("--data").arg(data);
        }
        if let Some(headers) = &headers {
            cmd.arg("--headers").arg(headers);
        }

        if let Some(xor) = &xor {
            cmd.arg("--xor").arg(xor.to_string());
        }

        let output = cmd.output().unwrap();

        String::from_utf8_lossy(&output.stdout).to_string()
    }
}

impl Drop for TestClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}