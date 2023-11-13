use super::*;

#[derive(Debug)]
pub struct Client {
    inner: nats::Client,
}

impl Client {
    pub fn new(client: nats::Client) -> Self {
        let inner = client.clone();
        Self { inner }
    }

    async fn request(&self, subject: String, payload: Bytes) -> Result<Bytes, nats::RequestError> {
        self.inner
            .request(subject, payload)
            .await
            .map(|message| message.payload)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn send<T: Send>() {}
    fn sync<T: Sync>() {}

    #[test]
    fn error_send_sync() {
        send::<Error>();
        sync::<Error>();
    }
}
