use futures::StreamExt;

use nats::service::endpoint::Endpoint;
use nats::service::error::Error;
use nats::service::Config;
use nats::service::Service;
use nats::service::ServiceExt;

use super::*;

use self::handle::Endpoints;

mod handle;

#[derive(Debug)]
pub struct Server {
    client: nats::Client,
    service: Service,
    endpoints: Endpoints,
}

impl Server {
    pub fn nats(&self) -> &nats::Client {
        &self.client
    }

    pub async fn new(client: nats::Client) -> Result<Self, nats::Error> {
        let description = option_env!("CARGO_PKG_DESCRIPTION").map(ToString::to_string);
        let name = env!("CARGO_PKG_NAME").to_string();
        let version = env!("CARGO_PKG_VERSION").to_string();
        let config = Config {
            name,
            description,
            version,
            stats_handler: None,
            metadata: None,
            queue_group: None,
        };

        Self::with_config(client, config).await
    }

    pub async fn with_config(client: nats::Client, config: Config) -> Result<Self, nats::Error> {
        let endpoints = Endpoints::default();
        client.add_service(config).await.map(|service| Self {
            client,
            service,
            endpoints,
        })
    }

    pub async fn method<R>(self, ctx: R) -> Result<Self, nats::Error>
    where
        R: Send
            + Sync
            + JsonRpc2
            + JsonRpc2Service<
                <R as JsonRpc2>::Request,
                Response = <R as JsonRpc2>::Response,
                Error = <R as JsonRpc2>::Error,
            > + 'static,
    {
        let endpoint = self.create_endpoint::<R>().await?;
        let endpoints = self.endpoints.endpoint(ctx, endpoint);
        Ok(Self { endpoints, ..self })
    }

    pub async fn create_endpoint<R>(&self) -> Result<Endpoint, nats::Error>
    where
        R: JsonRpc2,
    {
        self.service.endpoint(R::METHOD).await
    }

    pub async fn start_endpoint<R>(&self, mut endpoint: Endpoint, ctx: R)
    where
        R: JsonRpc2
            + JsonRpc2Service<
                <R as JsonRpc2>::Request,
                Response = <R as JsonRpc2>::Response,
                Error = <R as JsonRpc2>::Error,
            >,
    {
        while let Some(request) = endpoint.next().await {
            if let Err(error) = handle_nats_request(&ctx, request).await {
                tracing::error!(%error, "Failed to send response");
            }
        }
    }

    pub async fn run(self) {
        self.endpoints.run().await
    }

    pub async fn start_single_rpc_method<R>(&self, ctx: R) -> Result<(), nats::Error>
    where
        R: JsonRpc2
            + JsonRpc2Service<
                <R as JsonRpc2>::Request,
                Response = <R as JsonRpc2>::Response,
                Error = <R as JsonRpc2>::Error,
            >,
    {
        let endpoint = self.create_endpoint::<R>().await?;
        self.start_endpoint(endpoint, ctx).await;
        Ok(())
    }
}

async fn handle_nats_request<R>(
    ctx: &R,
    request: nats::service::Request,
) -> Result<(), nats::PublishError>
where
    R: JsonRpc2
        + JsonRpc2Service<
            <R as JsonRpc2>::Request,
            Response = <R as JsonRpc2>::Response,
            Error = <R as JsonRpc2>::Error,
        >,
{
    let response = handle_jsonrpc_call::<R>(ctx, &request.message.payload)
        .await
        .map_err(nats_service_error);
    request.respond(response).await
}

async fn handle_jsonrpc_call<R>(ctx: &R, request: &[u8]) -> json::Result<Bytes>
where
    R: JsonRpc2
        + JsonRpc2Service<
            <R as JsonRpc2>::Request,
            Response = <R as JsonRpc2>::Response,
            Error = <R as JsonRpc2>::Error,
        >,
{
    let (id, request) = json::from_slice::<jsonrpc::Request>(request)?.into_request::<R>()?;

    tracing::trace!(?request);
    let result = ctx.call(request).await;
    tracing::trace!(?result);

    let response = jsonrpc::Response::from_result(id, result)?;
    json::to_vec(&response).map(Bytes::from)
}

fn nats_service_error(error: json::Error) -> Error {
    Error {
        status: error.to_string(),
        code: usize::MAX,
    }
}
