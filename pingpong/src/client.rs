use super::*;

pub(super) async fn client(addrs: String, text: String, count: usize) -> anyhow::Result<()> {
    let client = Nats::new(addrs).await?.client();

    let r1 = pingpong::PingPongRequest::new(count, &text);
    let response = client.call::<pingpong::PingPong>(r1).await?;
    tracing::info!(?response);

    let r2 = pingpong::PingPongRequest::new(count - 1, &text);
    let response = client.call::<pingpong::PingPong>(r2).await?;
    tracing::info!(?response);

    Ok(())
}
