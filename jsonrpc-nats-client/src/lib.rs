use async_nats as nats;
use jsonrpc::client::AsyncClient;
use jsonrpc_nats::Nats;

pub async fn nats_client(
    addr: impl nats::ToServerAddrs,
) -> Result<AsyncClient<Nats>, nats::ConnectError> {
    Nats::new(addr).await.map(AsyncClient::with_transport)
}
