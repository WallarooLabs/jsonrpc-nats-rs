use handle::Endpoints;

use super::*;

mod endpoint_old;
mod handle;

#[derive(Debug)]
pub struct Server {
    // rx: mpsc::Receiver<Request>,
    service: Service,
    // handlers: HashMap<&'static str, Box<u8>>,
    endpoints: Endpoints,
}

impl Server {
    pub(crate) fn new(rx: mpsc::Receiver<Request>) -> Self {
        let service = Service::new(rx);
        let endpoints = Endpoints::default();
        Self { service, endpoints }
    }

    pub async fn method<R>(self, ctx: R) -> Result<Self, Error>
    where
        R: 'static
            + JsonRpc2
            + JsonRpc2Service<
                <R as JsonRpc2>::Request,
                Response = <R as JsonRpc2>::Response,
                Error = <R as JsonRpc2>::Error,
            >,
    {
        let endpoint = self.create_endpoint::<R>().await?;
        let endpoints = self.endpoints.endpoint(ctx, endpoint).await;
        Ok(Self { endpoints, ..self })
    }

    pub async fn add_method<R>(&self, ctx: R) -> Result<&Self, Error>
    where
        R: 'static
            + JsonRpc2
            + JsonRpc2Service<
                <R as JsonRpc2>::Request,
                Response = <R as JsonRpc2>::Response,
                Error = <R as JsonRpc2>::Error,
            >,
    {
        let endpoint = self.create_endpoint::<R>().await?;
        self.endpoints.add_endpoint(ctx, endpoint).await;
        Ok(self)
    }

    pub(crate) async fn create_endpoint<R>(&self) -> Result<Endpoint, Error>
    where
        R: JsonRpc2,
    {
        Ok(self.service.endpoint(R::METHOD).await)
    }

    pub async fn run(self) {
        let mut tasks = tokio::task::JoinSet::<anyhow::Result<()>>::new();
        let _aborts: Vec<tokio::task::AbortHandle> = self.spawn_on(&mut tasks).await;
        while let Some(done) = tasks.join_next().await {
            if let Err(err) = done {
                tracing::error!(%err, "join failed");
            }
        }
    }

    pub async fn spawn_on(
        &self,
        tasks: &mut tokio::task::JoinSet<anyhow::Result<()>>,
    ) -> Vec<tokio::task::AbortHandle> {
        self.service.spawn_on(tasks).await;
        self.endpoints.spawn_on(tasks).await
    }
}

async fn handle_ipc_request<R>(ctx: &R, request: json::Value) -> json::Result<json::Value>
where
    R: JsonRpc2
        + JsonRpc2Service<
            <R as JsonRpc2>::Request,
            Response = <R as JsonRpc2>::Response,
            Error = <R as JsonRpc2>::Error,
        >,
{
    let request = json::from_value(request)?;
    let response = jsonrpc::handle_jsonrpc_call(ctx, request).await?;
    json::to_value(response)
}
