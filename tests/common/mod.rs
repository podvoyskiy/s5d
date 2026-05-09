use std::process::{Command, Child};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub struct TestServer {
    child: Child,
    port: u16,
}

impl TestServer {
    pub fn start(port: u16, auth: Option<(String, String)>) -> Self {
        let binary_path = std::env::var("CARGO_BIN_EXE_s5d").unwrap();
        let mut cmd = Command::new(binary_path);
        cmd.arg("--port").arg(port.to_string());

        if let Some((user, pass)) = &auth {
            cmd.arg("--auth").arg(format!("{user}:{pass}"));
        }

        let child = cmd.spawn().unwrap();

        thread::sleep(Duration::from_millis(200));

        Self { child, port }
    }

    pub fn client(&self) -> TcpStream {
        TcpStream::connect(format!("127.0.0.1:{}", self.port)).unwrap()
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}