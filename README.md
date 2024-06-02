# Async Rust JSONRPC framework

The implementation is modeled on `tower::Service`, but does not use it directly.
The reason is that `tower::Service` is not `async` and hence is less ergonomic
than it could be otherwise.

NOTE: There is `tower-async`, but it requires nightly compiler.

## (VHLD) Very High Level Description

```Rust
pub trait JsonRpc2 {
    const METHOD: &'static str;
    type Request: fmt::Debug + Serialize + de::DeserializeOwned;
    type Response: fmt::Debug + Serialize + de::DeserializeOwned;
    type Error: fmt::Debug + Serialize + de::DeserializeOwned;
}

pub trait JsonRpc2Service<Request> {
    type Response;
    type Error;
    async fn call(&self, request: Request) -> Result<Self::Response, Self::Error>;
}

```

These two traits are what it takes for a consumer to implement the RPC
server and client.

See `pingpong` crate for an example of implementing it.
