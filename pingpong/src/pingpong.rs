use jsonrpc::JsonRpc2;
use jsonrpc::JsonRpc2Service;
use serde::{Deserialize, Serialize};

// pub(crate) use impls::PingPongExt;

mod impls;

#[derive(Debug, JsonRpc2)]
#[jsonrpc(method = "pingpong", error = "String", client)]
pub(crate) struct PingPong;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PingPongRequest {
    count: usize,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PingPongResponse {
    text: String,
    count: usize,
}

/// This can also be derived automatically
///
// impl JsonRpc2 for PingPong {
//     const METHOD: &'static str = "pingpong";
//     type Request = PingPongRequest;
//     type Response = PingPongResponse;
//     type Error = String;
// }

#[jsonrpc::async_trait(?Send)]
impl JsonRpc2Service<<Self as JsonRpc2>::Request> for PingPong {
    type Response = <Self as JsonRpc2>::Response;
    type Error = <Self as JsonRpc2>::Error;

    async fn call(
        &self,
        request: <Self as JsonRpc2>::Request,
    ) -> Result<Self::Response, Self::Error> {
        if request.text.len() < 6 {
            Ok(request.into())
        } else {
            Err(format!("cannot process: {}", request.text))
        }
    }
}

impl PingPongRequest {
    pub(crate) fn new(count: usize, text: impl ToString) -> Self {
        let text = text.to_string();
        Self { count, text }
    }
}

impl From<PingPongRequest> for PingPongResponse {
    fn from(ping: PingPongRequest) -> Self {
        let text = ping.text.repeat(ping.count);
        let count = ping.text.len();
        Self { text, count }
    }
}
