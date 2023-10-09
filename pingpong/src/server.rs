use jsonrpc_nats_server::Server;

use crate::pingpong;

pub(super) async fn server(addr: String) -> anyhow::Result<()> {
    let server = Server::new(addr).await.map_err(|e| anyhow::anyhow!(e))?;
    println!("Server {server:?}");
    let ep = server
        .add_method::<pingpong::PingPong>()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    server.start_endpoint::<pingpong::PingPong>(ep, &()).await;

    Ok(())
}
