use async_nats as nats;
use bytes::Bytes;
use jsonrpc::async_trait;
use jsonrpc::AsyncClient;
use jsonrpc::JsonRpc2;
use jsonrpc::JsonRpc2Service;
use serde_json as json;

pub use self::client::Client;
pub use self::server::Server;

mod client;
mod server;

/// `Nats` is a main entry point for JSONRPC over NATS.
/// It initializes connection to the NATS service and then can create either
/// JSONRPC client or server object.
///
#[derive(Debug)]
pub struct Nats {
    client: nats::Client,
}

impl Nats {
    /// New NATS connection using all the default settings
    ///
    pub async fn new(addrs: impl nats::ToServerAddrs) -> Result<Self, nats::ConnectError> {
        let options = nats::ConnectOptions::new();
        Self::with_options(options, addrs).await
    }

    /// New NATS connection with `ConnectionOptions` object allowing fine tuning
    /// of the different connectivity options
    ///
    pub async fn with_options(
        options: nats::ConnectOptions,
        addrs: impl nats::ToServerAddrs,
    ) -> Result<Self, nats::ConnectError> {
        options.connect(addrs).await.map(|client| Self { client })
    }

    /// Convert this object into JSONRPC client
    ///
    pub fn client(self) -> AsyncClient<Client> {
        let transport = Client::new(self.client);
        AsyncClient::with_transport(transport)
    }

    /// Convert this object into JSONRPC server
    ///
    pub async fn server(self) -> Result<Server, nats::Error> {
        Server::new(self.client).await
    }
}
