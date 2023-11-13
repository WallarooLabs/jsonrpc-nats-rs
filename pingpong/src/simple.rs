use jsonrpc::JsonRpc2;
use jsonrpc::JsonRpc2Service;
// use serde::{Deserialize, Serialize};

#[derive(Debug, Default, JsonRpc2)]
#[jsonrpc(
    method = "simple",
    request = "()",
    response = "String",
    // error = "SimpleError",
    client
)]
pub(crate) struct Simple;

// pub(crate) type SimpleRequest = ();

pub(crate) type SimpleError = ();

// impl JsonRpc2 for Simple {
//     const METHOD: &'static str = "simple";
//     type Request = SimpleRequest;
//     type Response = String;
//     type Error = SimpleError;
// }

impl JsonRpc2Service<<Self as JsonRpc2>::Request> for Simple {
    type Response = <Self as JsonRpc2>::Response;
    type Error = <Self as JsonRpc2>::Error;

    async fn call(
        &self,
        _request: <Self as JsonRpc2>::Request,
    ) -> Result<Self::Response, Self::Error> {
        Ok("hello".to_string())
    }
}
