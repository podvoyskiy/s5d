use std::process::{Command, Child};
use std::thread;
use std::time::Duration;

pub struct TestClient {
    child: Child
}

impl TestClient {
    pub fn start(target: &str) -> Self {
        let child = 
            Command::new("./../target/debug/s5d-client")
            .arg("--target").arg(target)
            .spawn().unwrap();

        thread::sleep(Duration::from_millis(200));

        Self { child }
    }
}

impl Drop for TestClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}