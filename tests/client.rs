use crate::support::{test_client::TestClient, test_proxy::TestProxy};

mod support;

#[test]
fn test_client() {
    let port: u16 = 33336;
    let _proxy = TestProxy::start(port, None);
    let _client = TestClient::start(&format!("127.0.0.1:{port}"), "https://httpbin.org");
}