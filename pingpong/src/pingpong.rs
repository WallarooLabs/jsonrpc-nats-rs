use std::task;

use futures::future;
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) struct PingPong;

impl jsonrpc::JsonRpc2 for PingPong {
    const METHOD: &'static str = "pingpong";

    type Request = PingPongRequest;

    type Response = PingPongResponse;

    type Error = String;
}

impl jsonrpc::JsonRpc2Client for PingPong {}

#[jsonrpc::async_trait]
impl jsonrpc::JsonRpc2Service for PingPong {
    type Context = ();

    async fn serve(
        _ctx: &Self::Context,
        request: Option<Self::Request>,
    ) -> Result<Option<Self::Response>, Self::Error> {
        Ok(request.map(Into::into))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PingPongRequest {
    count: usize,
    text: String,
}

impl PingPongRequest {
    pub(crate) fn new(count: usize, text: impl ToString) -> Self {
        let text = text.to_string();
        Self { count, text }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PingPongResponse {
    text: String,
    count: usize,
}

impl From<PingPongRequest> for PingPongResponse {
    fn from(ping: PingPongRequest) -> Self {
        let text = ping.text.repeat(ping.count);
        let count = ping.text.len();
        Self { text, count }
    }
}

impl tower::Service<PingPongRequest> for PingPong {
    type Response = PingPongResponse;

    type Error = ();

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: PingPongRequest) -> Self::Future {
        let response = future::ok(PingPongResponse::from(request));
        Box::pin(response)
    }
}
