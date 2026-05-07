use std::{io::{Read, Write}, net::TcpStream, process::{Child, Command}, thread, time::Duration};

struct TestProxy {
    child: Child,
    port: u16,
}

impl TestProxy {
    fn start(port: u16, auth: Option<(String, String)>) -> Self {
        let mut cmd = Command::new("./target/debug/s5d");
        cmd.arg("--port").arg(port.to_string());

        if let Some((user, pass)) = &auth {
            cmd.arg("--auth").arg(format!("{user}:{pass}"));
        }

        let child = cmd.spawn().unwrap();

        thread::sleep(Duration::from_millis(200));

        Self { child, port }
    }

    fn client(&self) -> TcpStream {
        TcpStream::connect(format!("127.0.0.1:{}", self.port)).unwrap()
    }
}

impl Drop for TestProxy {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn test_proxy_handshake() {
    let proxy = TestProxy::start(33333, None);
    let mut client = proxy.client();

    client.write_all(&[0x05, 0x01, 0x00]).unwrap();
    let mut buf = [0; 2];
    client.read_exact(&mut buf).unwrap();

    assert_eq!(&buf, &[0x05, 0x00]);
}

#[test]
fn test_proxy_connect() {
    let proxy = TestProxy::start(33334, None);
    let mut client = proxy.client();

    // handshake
    client.write_all(&[0x05, 0x01, 0x00]).unwrap();
    let mut buf = [0; 2];
    client.read_exact(&mut buf).unwrap();

    // connect
    client.write_all(&[
        0x05, 0x01, 0x00, 0x03,
        0x0b, // domain length: 11 bytes
        b'h', b't', b't', b'p', b'b', b'i', b'n', b'.', b'o', b'r', b'g', // httpbin.org
        0x01, 0xbb // port: 443
    ]).unwrap();
    let mut buf = [0; 10];
    client.read_exact(&mut buf).unwrap();

    assert_eq!(buf[0], 0x05);
    assert_eq!(buf[1], 0x00);
}

#[test]
fn test_proxy_auth() {
    let username = String::from("admin");
    let password = String::from("12345");

    let proxy = TestProxy::start(33335, Some((username.clone(), password.clone())));
    let mut client = proxy.client();

    // handshake
    client.write_all(&[0x05, 0x01, 0x02]).unwrap();
    let mut buf = [0; 2];
    client.read_exact(&mut buf).unwrap();

    assert_eq!(&buf, &[0x05, 0x02]);

    // auth
    let mut response = Vec::with_capacity(1 + 1 + username.len() + 1 + password.len());
    response.push(0x01); // auth version
    response.push(username.len() as u8); // ulen
    response.extend(username.as_bytes()); // uname
    response.push(password.len() as u8); // plen
    response.extend(password.as_bytes()); // passwd

    client.write_all(&response).unwrap();
    let mut buf = [0; 2];
    client.read_exact(&mut buf).unwrap();

    assert_eq!(&buf, &[0x01, 0x00]);
}