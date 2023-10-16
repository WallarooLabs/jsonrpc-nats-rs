use super::*;

#[async_trait(?Send)]
pub trait JsonRpc2Service<Request> {
    type Response;
    type Error;
    async fn call(&self, request: Request) -> Result<Self::Response, Self::Error>;
}
