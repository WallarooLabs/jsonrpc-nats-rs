use async_nats as nats;
use jsonrpc::async_trait;
use jsonrpc::TransportError;
use serde_json as json;

mod server;

#[derive(Debug)]
pub struct Nats {
    client: nats::Client,
}

impl Nats {
    pub async fn new(addr: impl nats::ToServerAddrs) -> Result<Self, nats::ConnectError> {
        nats::connect(addr).await.map(|client| Self { client })
    }
}

#[async_trait]
impl jsonrpc::ClientTransport for Nats {
    type TransportError = nats::client::RequestError;
    type ResponseHandle = nats::Message;

    async fn send_request(
        &mut self,
        request: &jsonrpc::Request,
    ) -> Result<Self::ResponseHandle, jsonrpc::Error<Self::TransportError>> {
        let subject = request.method.to_string();
        let payload = json::to_vec(request)?.into();
        self.client
            .request(subject, payload)
            .await
            .transport_error()
    }

    async fn recv_response(
        &mut self,
        handle: Self::ResponseHandle,
    ) -> Result<jsonrpc::Response, jsonrpc::Error<Self::TransportError>> {
        let response = json::from_slice(&handle.payload)?;
        Ok(response)
    }
}
