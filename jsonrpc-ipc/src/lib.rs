use std::cell::Cell;

use jsonrpc::AsyncClient;
use jsonrpc::JsonRpc2;
use jsonrpc::JsonRpc2Service;
use serde_json as json;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub use self::client::Client;
pub use self::request::Request;
pub use self::server::Server;
pub use self::service::Endpoint;
pub use self::service::Service;

mod client;
mod impls;
mod request;
mod server;
mod service;

// type TransportPayload = (json::Value, oneshot::Sender<json::Value>);

/// `Ipc` is a main entry point for JSONRPC over IPC.
/// It initializes internal framework and then can create either
/// JSONRPC client or server object.
///
pub struct Ipc {
    tx: mpsc::Sender<Request>,
    rx: Cell<Option<mpsc::Receiver<Request>>>,
}

impl Ipc {
    /// New IPC connection using all the default settings
    ///
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Request>(10);
        let rx = Cell::new(Some(rx));
        Self { tx, rx }
    }

    /// Convert this object into JSONRPC client
    ///
    pub fn client(self) -> AsyncClient<Client> {
        let transport = Client::new(self.tx.clone());
        AsyncClient::with_transport(transport)
    }

    /// Convert this object into JSONRPC server
    ///
    pub async fn server(self) -> Result<Server, Error> {
        self.rx
            .take()
            .ok_or(Error::ServerAlreadyExists)
            .map(Server::new)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic Failure")]
    TransportFailure,
    #[error("JSON error {0}")]
    JsonError(#[from] json::Error),
    #[error("Server already instantiated")]
    ServerAlreadyExists,
    #[error("Client is gone")]
    ClientIsGone,
}

impl From<mpsc::error::SendError<Request>> for Error {
    fn from(error: mpsc::error::SendError<Request>) -> Self {
        tracing::warn!(?error);
        Self::TransportFailure
    }
}

impl From<oneshot::error::RecvError> for Error {
    fn from(error: oneshot::error::RecvError) -> Self {
        tracing::warn!(?error);
        Self::TransportFailure
    }
}
