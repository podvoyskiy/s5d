mod support;

use crate::support::test_client::TestClient;

#[test]
fn test_client_handshake() {
    let _client = TestClient::start("https://httpbin.org");
}