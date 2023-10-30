use jsonrpc_nats::Server;

use super::*;

pub(super) async fn server(addrs: String) -> anyhow::Result<()> {
    let server = Nats::new(addrs)
        .await?
        .server()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    tracing::info!(?server, "Starting");

    multiple(&server).await
}

async fn multiple(server: &Server) -> anyhow::Result<()> {
    let pp = pingpong::PingPong;
    let c = count::Count::default();
    let s = simple::Simple;

    let mut ep1 = server
        .add_method::<pingpong::PingPong>()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    let mut ep2 = server
        .add_method::<count::Count>()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    let mut ep3 = server
        .add_method::<simple::Simple>()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    loop {
        tokio::select! {
            Some(_) = Server::serve_one(&mut ep1, &pp) => {},
            Some(_) = Server::serve_one(&mut ep2, &c) => {},
            Some(_) = Server::serve_one(&mut ep3, &s) => {},
        }
    }
}

async fn _single(server: &Server) -> anyhow::Result<()> {
    let ctx = count::Count::default();

    server
        .start_single_rpc_method(ctx)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
