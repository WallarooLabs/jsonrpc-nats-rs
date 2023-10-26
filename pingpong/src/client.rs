use super::*;

use count::CountExt;
use pingpong::PingPongExt;
use simple::SimpleExt;

pub(super) async fn client(addrs: String, method: Method) -> anyhow::Result<()> {
    let client = Nats::new(addrs).await?.client();

    match method {
        Method::Count => {
            let r = count::CountRequest;
            let response = client.count(r).await?;
            tracing::info!(?response);
        }
        Method::Ping { text, count } => {
            let r1 = pingpong::PingPongRequest::new(count, &text);
            let response = client.call::<pingpong::PingPong>(r1).await?;
            tracing::info!(?response);

            let r2 = pingpong::PingPongRequest::new(count - 1, &text);
            let response = client.pingpong(r2).await?;
            tracing::info!(?response);
        }
        Method::Simple => {
            let response = client.simple(()).await?;
            tracing::info!(?response);
        }
    }

    Ok(())
}
