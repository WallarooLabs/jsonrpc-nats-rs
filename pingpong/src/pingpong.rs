use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) struct PingPong;

impl jsonrpc::JsonRpc2 for PingPong {
    const METHOD: &'static str = "pingpong";
    type Request = PingPongRequest;
    type Response = PingPongResponse;
    type Error = String;
}

#[jsonrpc::async_trait(?Send)]
impl jsonrpc::service::JsonRpc2Service<<Self as jsonrpc::JsonRpc2>::Request> for PingPong {
    type Response = <Self as jsonrpc::JsonRpc2>::Response;
    type Error = <Self as jsonrpc::JsonRpc2>::Error;

    async fn call(
        &self,
        request: <Self as jsonrpc::JsonRpc2>::Request,
    ) -> Result<Self::Response, Self::Error> {
        if request.text.len() < 6 {
            Ok(request.into())
        } else {
            Err(format!("cannot process: {}", request.text))
        }
    }
}

#[jsonrpc::async_trait]
impl jsonrpc::JsonRpc2Service for PingPong {
    type Context = ();

    async fn serve(
        _ctx: &Self::Context,
        request: Option<Self::Request>,
    ) -> Result<Option<Self::Response>, Self::Error> {
        if let Some(request) = request {
            if request.text.len() < 6 {
                Ok(Some(request.into()))
            } else {
                Err(format!("cannot process: {}", request.text))
            }
        } else {
            Err(String::from("Request is mandatory"))
        }
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
