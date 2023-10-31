use super::*;

/// Service trait
/// It is closely related to `JsonRpc2` trait. The `Request`, `Response` and `Error`
/// types are those of the `JsonRpc2`.
/// Server implements the `call` functionality, while client uses `call` to
/// initiate the the request-response exchange.
///
#[async_trait]
pub trait JsonRpc2Service<Request>: Send + Sync {
    type Response: Send + Sync;
    type Error: Send + Sync;
    async fn call(&self, request: Request) -> Result<Self::Response, Self::Error>;
}
