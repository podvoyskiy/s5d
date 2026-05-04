use std::{io::{Read, Write}, net::TcpStream, process::{Child, Command}, thread, time::Duration};

struct TestProxy {
    child: Child,
    port: u16
}

impl TestProxy {
    fn start(port: u16) -> Self {
        let child = Command::new("./target/debug/s5d")
            .arg("--port")
            .arg(port.to_string())
            .spawn()
            .unwrap();

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
    let proxy = TestProxy::start(33333);
    let mut client = proxy.client();

    client.write_all(&[0x05, 0x01, 0x00]).unwrap();
    let mut buf = [0; 2];
    client.read_exact(&mut buf).unwrap();

    assert_eq!(&buf, &[0x05, 0x00]);
}

#[test]
fn test_proxy_connect() {
    let proxy = TestProxy::start(33334);
    let mut client = proxy.client();

    //handshake
    client.write_all(&[0x05, 0x01, 0x00]).unwrap();
    let mut buf = [0; 2];
    client.read_exact(&mut buf).unwrap();

    //connect
    let request = &[
        0x05, 0x01, 0x00, 0x03,
        0x0b, //domain length: 11 bytes
        b'h', b't', b't', b'p', b'b', b'i', b'n', b'.', b'o', b'r', b'g', //httpbin.org
        0x01, 0xbb //port: 443
    ];
    client.write_all(request).unwrap();
    let mut buf = [0; 10];
    client.read_exact(&mut buf).unwrap();
    println!("{:?}", buf);

    assert_eq!(buf[0], 0x05);
    assert_eq!(buf[1], 0x00);
}