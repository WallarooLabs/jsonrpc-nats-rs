# Integration Tests

This directory contains integration tests for the jsonrpc-nats library that verify compatibility with the async-nats dependency.

## Running Tests

### Prerequisites

Tests require a running NATS server. You have several options:

#### Option 1: Using Docker (Recommended)
```bash
# Start NATS server
docker run -d -p 4222:4222 --name nats-test nats:latest

# Run tests
cargo test -p jsonrpc-nats --test integration_test

# Stop NATS server
docker stop nats-test && docker rm nats-test
```

#### Option 2: Using justfile (Easiest)
```bash
# Automatically start NATS, run tests, and cleanup
just test-full

# Or run integration tests with already-running NATS
just test-integration
```

#### Option 3: Local NATS Installation
If you have nats-server installed locally:
```bash
# Start NATS server
nats-server

# In another terminal, run tests
cargo test -p jsonrpc-nats --test integration_test
```

### Using Custom NATS URL

By default, tests connect to `nats://localhost:4222`. To use a different NATS server:

```bash
NATS_URL=nats://my-nats-server:4222 cargo test -p jsonrpc-nats --test integration_test
```

## Test Coverage

The integration tests cover:

1. **Basic Connectivity** (`test_nats_connection`)
   - Verifies connection to NATS server
   - Validates connection options

2. **Publish/Subscribe** (`test_nats_publish_subscribe`)
   - Tests core NATS pub/sub functionality
   - Ensures message delivery

3. **JSON-RPC Server/Client** (`test_jsonrpc_server_client`)
   - End-to-end RPC call testing
   - Server endpoint registration
   - Client request/response handling

4. **Service API** (`test_service_endpoint_creation`)
   - Service endpoint creation
   - NATS Service API integration

5. **Error Handling** (`test_request_error_handling`)
   - Timeout handling
   - No responders scenarios
   - Error propagation

6. **Concurrency Safety** (`test_client_is_send_sync`, `test_server_is_send_sync`)
   - Ensures types are Send + Sync
   - Validates thread safety

## Continuous Integration

For CI environments without Docker, tests will automatically skip if NATS server is not available:

```bash
# Tests will output: "Skipping test: NATS server not available"
cargo test -p jsonrpc-nats --test integration_test
```

## Troubleshooting

### Tests Timeout
- Ensure NATS server is running and accessible
- Check firewall settings
- Verify NATS server is listening on the expected port

### Connection Refused
```
Error: connection refused
```
- NATS server is not running
- Wrong host/port configuration
- Firewall blocking connection

### All Tests Skipped
```
Skipping test: NATS server not available
```
- This is normal when NATS server is not running
- Tests will pass as "skipped"
- To run tests: start NATS server first

## Test Structure

Each test follows this pattern:

```rust
#[tokio::test]
async fn test_name() {
    // Get NATS server URL (defaults to localhost:4222)
    let server_url = get_nats_server().await;

    // Try to connect
    let nats = match Nats::new(&server_url).await {
        Ok(n) => n,
        Err(_) => {
            // Skip if NATS not available
            eprintln!("Skipping test: NATS server not available");
            return;
        }
    };

    // Test logic here...
}
```

## Upgrade Testing

These tests are specifically designed to catch breaking changes when upgrading async-nats:

**Before upgrading:**
```bash
# Ensure tests pass with current version
just test-full
```

**After upgrading:**
```bash
# Update dependency in Cargo.toml
# Run tests again to verify compatibility
just test-full
```

If tests fail after upgrade:
1. Check error messages for API changes
2. Review UPGRADE_ASSESSMENT.md
3. Check async-nats CHANGELOG
4. Update code as needed

## Adding New Tests

To add new integration tests:

1. Add test function to `integration_test.rs`
2. Use `#[tokio::test]` attribute
3. Follow the skip-if-unavailable pattern
4. Document what the test verifies
5. Run with `just test-integration`

Example:
```rust
#[tokio::test]
async fn test_my_new_feature() {
    let server_url = get_nats_server().await;

    let nats = match Nats::new(&server_url).await {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Skipping test: NATS server not available");
            return;
        }
    };

    // Your test code...
    assert!(true);
}
```
