use jsonrpc_nats::Server;

use super::*;

pub(crate) async fn server(addrs: String) -> anyhow::Result<()> {
    let pp = pingpong::PingPong;
    let c = count::Count::default();
    let s = simple::Simple;

    Nats::new(addrs)
        .await?
        .server()
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .method(pp)
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .method(c)
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .method(s)
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .run()
        .await;

    Ok(())
}

pub(super) async fn _server(addrs: String) -> anyhow::Result<()> {
    let server = Nats::new(addrs)
        .await?
        .server()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    tracing::info!(?server, "Starting");

    _single(&server).await
}

async fn _single(server: &Server) -> anyhow::Result<()> {
    let ctx = count::Count::default();

    server
        .start_single_rpc_method(ctx)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
