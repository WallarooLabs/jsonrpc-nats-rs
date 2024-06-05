use super::*;

#[derive(Debug)]
pub struct Request {
    pub message: json::Value,
    tx: Option<oneshot::Sender<json::Value>>,
}

impl Request {
    pub(crate) fn new(message: json::Value, tx: oneshot::Sender<json::Value>) -> Self {
        let tx = Some(tx);
        Self { message, tx }
    }

    pub async fn respond(&mut self, response: json::Value) -> Result<(), Error> {
        self.tx
            .take()
            .expect("This request has been responded already")
            .send(response)
            .inspect_err(|_| tracing::error!("Failed to send back response; caller disappeared?"))
            .map_err(|_| Error::ClientIsGone)
    }
}
