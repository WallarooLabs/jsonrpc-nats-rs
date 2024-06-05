use std::future::Future;

use super::*;

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

pub async fn handle_jsonrpc_call<R>(ctx: &R, request: Request) -> json::Result<Response>
where
    R: JsonRpc2
        + JsonRpc2Service<
            <R as JsonRpc2>::Request,
            Response = <R as JsonRpc2>::Response,
            Error = <R as JsonRpc2>::Error,
        >,
{
    let (id, request) = request.into_request::<R>()?;
    tracing::trace!(?request);
    let result = ctx.call(request).await;
    tracing::trace!(?result);
    Response::from_result(id, result)
}
