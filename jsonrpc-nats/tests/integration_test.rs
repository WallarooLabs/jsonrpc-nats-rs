use bytes::Bytes;
use futures::StreamExt;
use jsonrpc::{JsonRpc2, JsonRpc2Service};
use jsonrpc_nats::Nats;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

/// Test RPC method for ping-pong
#[derive(Debug)]
struct TestPing;

impl JsonRpc2 for TestPing {
    const METHOD: &'static str = "test.ping";
    type Request = PingRequest;
    type Response = PingResponse;
    type Error = PingError;
}

#[derive(Debug, Serialize, Deserialize)]
struct PingRequest {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PingResponse {
    reply: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PingError {
    code: i32,
    message: String,
}

impl JsonRpc2Service<PingRequest> for TestPing {
    type Response = PingResponse;
    type Error = PingError;

    async fn call(&self, request: PingRequest) -> Result<Self::Response, Self::Error> {
        Ok(PingResponse {
            reply: format!("pong: {}", request.message),
        })
    }
}

/// Helper function to start a NATS server (requires nats-server to be installed)
/// For CI/testing, you can use Docker or embedded nats-server
async fn get_nats_server() -> String {
    // Default to local NATS server
    std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string())
}

#[tokio::test]
async fn test_nats_connection() {
    let server_url = get_nats_server().await;

    // Test basic connection
    let result = Nats::new(&server_url).await;

    // If NATS server is not available, skip the test
    if result.is_err() {
        eprintln!("Skipping test: NATS server not available at {}", server_url);
        return;
    }

    let nats = result.unwrap();
    let _client = nats.client();

    // Connection successful - test passes if we get here
}

#[tokio::test]
async fn test_nats_publish_subscribe() {
    let server_url = get_nats_server().await;

    let server_nats = match Nats::new(&server_url).await {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Skipping test: NATS server not available");
            return;
        }
    };

    // Create server to get access to nats client
    let server = server_nats.server().await.unwrap();
    let nats_client = server.nats();

    // Subscribe to a test subject
    let mut sub = nats_client.subscribe("test.subject").await.unwrap();

    // Publish a message
    nats_client
        .publish("test.subject", Bytes::from("test message"))
        .await
        .unwrap();

    // Receive the message with timeout
    let msg = timeout(Duration::from_secs(2), sub.next())
        .await
        .expect("Timeout waiting for message")
        .expect("No message received");

    assert_eq!(msg.payload, Bytes::from("test message"));
}

#[tokio::test]
async fn test_jsonrpc_server_client() {
    let server_url = get_nats_server().await;

    let server_nats = match Nats::new(&server_url).await {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Skipping test: NATS server not available");
            return;
        }
    };

    // Start server
    let server = server_nats.server().await.unwrap();
    let server = server.method(TestPing).await.unwrap();

    // Spawn server task
    let server_handle = tokio::spawn(async move {
        server.run().await;
    });

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Create client
    let client_nats = Nats::new(&server_url).await.unwrap();
    let client = client_nats.client();

    // Make request using the proper API
    let request = PingRequest {
        message: "hello".to_string(),
    };

    let response = timeout(
        Duration::from_secs(2),
        client.call::<TestPing>(Some(request)),
    )
    .await
    .expect("Request timeout")
    .expect("Transport error")
    .expect("RPC error");

    // Verify response
    assert_eq!(response.reply, "pong: hello");

    // Cleanup
    server_handle.abort();
}

#[tokio::test]
async fn test_service_endpoint_creation() {
    let server_url = get_nats_server().await;

    let nats = match Nats::new(&server_url).await {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Skipping test: NATS server not available");
            return;
        }
    };

    let server = nats.server().await.unwrap();

    // Test that we can create an endpoint
    let endpoint = server.create_endpoint::<TestPing>().await;
    assert!(endpoint.is_ok());
}

#[tokio::test]
async fn test_request_error_handling() {
    let server_url = get_nats_server().await;

    let nats = match Nats::new(&server_url).await {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Skipping test: NATS server not available");
            return;
        }
    };

    let client = nats.client();

    // Make request to non-existent service (should timeout or error)
    let request = PingRequest {
        message: "test".to_string(),
    };

    // This should fail with a timeout or no responders error
    let result = timeout(
        Duration::from_secs(1),
        client.call::<TestPing>(Some(request)),
    )
    .await;

    // We expect either a timeout or an error - both are acceptable
    match result {
        Err(_) => {
            // Timeout is expected when no server is listening - test passes
        }
        Ok(Err(_)) => {
            // Error from NATS (e.g., no responders) is also acceptable - test passes
        }
        Ok(Ok(_)) => {
            // Should not succeed
            panic!("Expected error or timeout, got success");
        }
    }
}

#[test]
fn test_client_is_send_sync() {
    use jsonrpc_nats::Client;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Client>();
    assert_sync::<Client>();
}

#[test]
fn test_server_is_send_sync() {
    use jsonrpc_nats::Server;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Server>();
    assert_sync::<Server>();
}
