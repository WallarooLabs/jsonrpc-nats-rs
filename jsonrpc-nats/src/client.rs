use super::*;

#[derive(Debug)]
pub struct Client {
    inner: nats::Client,
}

impl Client {
    pub async fn new(addr: impl nats::ToServerAddrs) -> Result<Self, nats::ConnectError> {
        nats::connect(addr)
            .await
            .map(|client| Self { inner: client })
    }

    pub async fn client(
        addr: impl nats::ToServerAddrs,
    ) -> Result<AsyncClient<Self>, nats::ConnectError> {
        Self::new(addr).await.map(AsyncClient::with_transport)
    }

    pub async fn request(
        &self,
        subject: String,
        payload: Bytes,
    ) -> Result<Bytes, nats::RequestError> {
        self.inner
            .request(subject, payload)
            .await
            .map(|message| message.payload)
    }
}

#[async_trait(?Send)]
impl JsonRpc2Service<jsonrpc::Request> for Client {
    type Response = jsonrpc::Response;
    type Error = Error;

    async fn call(&self, request: jsonrpc::Request) -> Result<Self::Response, Self::Error> {
        let subject = request.method.to_string();
        let payload = json::to_vec(&request)?.into();
        let response = self.request(subject, payload).await?;
        let response = json::from_slice(&response)?;

        Ok(response)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Transport error")]
    TransportError(#[from] nats::RequestError),
    #[error("JSON error")]
    JsonError(#[from] json::Error),
}
