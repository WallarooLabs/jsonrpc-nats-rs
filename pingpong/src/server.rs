use super::*;

pub(super) async fn server(addrs: String) -> anyhow::Result<()> {
    let server = Nats::new(addrs)
        .await?
        .server()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    tracing::info!(?server, "Starting");

    // let ctx = pingpong::PingPong;
    let ctx = count::Count::default();

    server
        .start_single_rpc_method(ctx)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
