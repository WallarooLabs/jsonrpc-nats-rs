use super::*;

pub(super) async fn server(addrs: String) -> anyhow::Result<()> {
    let server = Nats::new(addrs)
        .await?
        .server()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    tracing::info!(?server, "Starting");

    let ctx = pingpong::PingPong;

    let ep = server
        .add_method::<pingpong::PingPong>()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    server.start_endpoint(ep, ctx).await;

    Ok(())
}
