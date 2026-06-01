use crate::support::{test_client::TestClient, test_proxy::TestProxy};

mod support;

#[test]
fn test_client() {
    let port: u16 = 33336;
    let _proxy = TestProxy::start(port, None);
    let _client = TestClient::start(&format!("127.0.0.1:{port}"), "http://httpbin.org/get");
}

#[test]
fn test_http() {
    let port: u16 = 33337;
    let _proxy = TestProxy::start(port, None);
    let output = TestClient::run(
        &format!("127.0.0.1:{port}"), 
        "http://httpbin.org/post", 
        Some("{\"key\":\"value\"}"), 
        None
    );

    assert!(output.contains("200 OK"));
    assert!(output.contains("\"url\": \"http://httpbin.org/post\""));
    assert!(output.contains("\"key\": \"value\""));
}

#[test]
fn test_https() {
    let port: u16 = 33338;
    let _proxy = TestProxy::start(port, None);
    let output = TestClient::run(
        &format!("127.0.0.1:{port}"), 
        "https://httpbin.org/post", 
        Some("{\"key\":\"value\"}"),
        Some("Auth:qwerty123"),
    );

    assert!(output.contains("200 OK"));
    assert!(output.contains("\"url\": \"https://httpbin.org/post\""));
    assert!(output.contains("\"key\": \"value\""));
    assert!(output.contains("\"Auth\": \"qwerty123\""));
}