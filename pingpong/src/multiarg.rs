use jsonrpc::JsonRpc2;
use jsonrpc::JsonRpc2Service;
// use serde::{Deserialize, Serialize};

#[derive(Debug, Default, JsonRpc2)]
#[jsonrpc(
    method = "multi",
    request = "(String, usize)",
    response = "String",
    error = "()",
    client
)]
pub(crate) struct Multiarg {
    seed: String,
}

// impl JsonRpc2 for Multiarg {
//     const METHOD: &'static str = "multi";
//     type Request = (String, usize);
//     type Response = String;
//     type Error = String;
// }

impl JsonRpc2Service<<Self as JsonRpc2>::Request> for Multiarg {
    type Response = <Self as JsonRpc2>::Response;
    type Error = <Self as JsonRpc2>::Error;

    async fn call(
        &self,
        (text, count): <Self as JsonRpc2>::Request,
    ) -> Result<Self::Response, Self::Error> {
        let text = if text.is_empty() { &self.seed } else { &text };
        Ok(text.repeat(count))
    }
}
