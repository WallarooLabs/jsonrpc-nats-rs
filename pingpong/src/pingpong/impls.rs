use super::*;

#[expect(dead_code)]
pub(crate) trait PingPongExt<T>
where
    T: jsonrpc::JsonRpc2Service<jsonrpc::Request, Response = jsonrpc::Response>,
    T::Error: From<serde_json::Error>,
{
    async fn pingpong(
        &self,
        count: usize,
        text: String,
    ) -> Result<Result<PingPongResponse, String>, T::Error>;
}

impl<T> PingPongExt<T> for jsonrpc::AsyncClient<T>
where
    T: jsonrpc::JsonRpc2Service<jsonrpc::Request, Response = jsonrpc::Response>,
    T::Error: From<serde_json::Error>,
{
    async fn pingpong(
        &self,
        count: usize,
        text: String,
    ) -> Result<Result<PingPongResponse, String>, T::Error> {
        let request = PingPongRequest { count, text };
        self.call::<PingPong>(request).await
    }
}
