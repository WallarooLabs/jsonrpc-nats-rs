use jsonrpc_nats::Client;

use super::*;

pub(super) async fn client(addr: String, text: String, count: usize) -> anyhow::Result<()> {
    let client = Client::client(addr).await?;
    println!("{client:?}");
    let r1 = pingpong::PingPongRequest::new(count, text);
    let response = client.call::<pingpong::PingPong>(r1).await;
    println!("{response:?}");
    Ok(())
}
