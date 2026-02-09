alias b := build
build:
    cargo build --workspace

alias t := test
test:
    cargo test --workspace

# Run integration tests (requires NATS server running)
test-integration:
    @echo "Make sure NATS server is running on localhost:4222"
    @echo "You can start it with: docker run -d -p 4222:4222 --name nats-test nats:latest"
    cargo test -p jsonrpc-nats --test integration_test

# Start NATS server for testing
nats-up:
    docker run -d -p 4222:4222 --name nats-test nats:latest
    @echo "NATS server started. Waiting for it to be ready..."
    @sleep 2

# Stop NATS test server
nats-down:
    docker stop nats-test || true
    docker rm nats-test || true

# Run integration tests with automatic NATS server setup/teardown
test-full: nats-up
    cargo test -p jsonrpc-nats --test integration_test
    just nats-down

# Test the pingpong example (requires NATS server)
test-example:
    @echo "Starting server..."
    cargo run -p pingpong -- server &
    @sleep 2
    @echo "Running client..."
    cargo run -p pingpong -- client ping "test" 3
    @pkill -f "cargo run -p pingpong -- server" || true

alias c:=clippy
clippy:
    cargo clippy --workspace --all-targets

clean:
    cargo clean

update:
    cargo update

deprecated:
    cargo clippy --features clap/deprecated

fresh: clean update clippy test build

deps:
    cargo update && git commit -m "Update deps" Cargo.lock

expand:
    cargo expand -Z macro-backtrace
