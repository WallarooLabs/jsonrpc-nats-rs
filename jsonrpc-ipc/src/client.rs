use super::*;

#[derive(Clone, Debug)]
pub struct Client {
    tx: mpsc::Sender<Request>,
}

impl Client {
    pub(crate) fn new(tx: mpsc::Sender<Request>) -> Self {
        Self { tx }
    }

    async fn request(&self, request: jsonrpc::Request) -> Result<jsonrpc::Response, Error> {
        let message = json::to_value(request)?;
        let (tx, rx) = oneshot::channel();
        let request = Request::new(message, tx);
        self.tx.send(request).await?;
        let response = rx.await?;
        let response = json::from_value(response)?;
        Ok(response)
    }
}

impl JsonRpc2Service<jsonrpc::Request> for Client {
    type Response = jsonrpc::Response;
    type Error = Error;

    async fn call(&self, request: jsonrpc::Request) -> Result<Self::Response, Self::Error> {
        self.request(request).await
    }
}
