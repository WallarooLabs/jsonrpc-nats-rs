use jsonrpc_nats::client::Client;
// use jsonrpc_nats::Nats;

use super::*;

pub(super) async fn client(addr: String, text: String, count: usize) -> anyhow::Result<()> {
    // let mut client = Nats::client(addr).await?;
    let client = Client::client(addr).await?;
    println!("{client:?}");
    let r1 = pingpong::PingPongRequest::new(count, text);
    let response = client.call::<pingpong::PingPong>(r1).await;
    println!("{response:?}");
    Ok(())
}
