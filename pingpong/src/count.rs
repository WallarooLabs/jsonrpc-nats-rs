use std::sync::atomic::{AtomicU64, Ordering};

use jsonrpc::JsonRpc2;
use jsonrpc::JsonRpc2Service;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, JsonRpc2)]
#[jsonrpc(
    method = "count",
    // request = "CountRequest",
    // response = "CountResponse",
    // error = "CountError",
    client
)]
pub(crate) struct Count {
    count: AtomicU64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CountRequest;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CountResponse {
    count: u64,
}

pub(crate) type CountError = ();

// impl JsonRpc2 for Count {
//     const METHOD: &'static str = "count";
//     type Request = CountRequest;
//     type Response = CountResponse;
//     type Error = CountError;
// }

#[jsonrpc::async_trait(?Send)]
impl JsonRpc2Service<<Self as JsonRpc2>::Request> for Count {
    type Response = <Self as JsonRpc2>::Response;
    type Error = <Self as JsonRpc2>::Error;

    async fn call(
        &self,
        _request: <Self as JsonRpc2>::Request,
    ) -> Result<Self::Response, Self::Error> {
        let count = self.count.fetch_add(1, Ordering::Relaxed);
        Ok(CountResponse { count })
    }
}
