use std::future::Future;

/// Service trait
/// It is closely related to `JsonRpc2` trait. The `Request`, `Response` and `Error`
/// types are those of the `JsonRpc2`.
/// Server implements the `call` functionality, while client uses `call` to
/// initiate the the request-response exchange.
///
pub trait JsonRpc2Service<Request>: Send + Sync {
    type Response: Send + Sync;
    type Error: Send + Sync;
    fn call(
        &self,
        request: Request,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}
