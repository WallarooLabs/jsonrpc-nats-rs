use futures::StreamExt;

use nats::service::endpoint::Endpoint;
use nats::service::ServiceExt;

use super::*;

#[derive(Debug)]
pub struct Server {
    client: nats::Client,
    service: nats::service::Service,
}

impl Server {
    pub fn nats(&self) -> &nats::Client {
        &self.client
    }

    pub async fn new(addr: impl nats::ToServerAddrs) -> Result<Self, nats::Error> {
        let client = async_nats::connect(addr).await?;
        let description = option_env!("CARGO_PKG_DESCRIPTION").map(ToString::to_string);
        let name = env!("CARGO_PKG_NAME").to_string();
        let version = env!("CARGO_PKG_VERSION").to_string();
        let config = nats::service::Config {
            name,
            description,
            version,
            stats_handler: None,
            metadata: None,
            queue_group: None,
        };
        let service = client.add_service(config).await?;
        Ok(Self { client, service })
    }

    pub async fn add_method<R>(&self) -> Result<Endpoint, nats::Error>
    where
        R: JsonRpc2Service,
    {
        let endpoint = self.service.endpoint(R::METHOD).await?;
        Ok(endpoint)
    }

    pub async fn start_endpoint<R>(&self, mut ep: Endpoint, ctx: &R::Context)
    where
        R: JsonRpc2Service,
    {
        while let Some(request) = ep.next().await {
            let response = handle_one_request::<R>(ctx, &request)
                .await
                .map_err(nats_service_error);
            if let Err(publish) = request.respond(response).await {
                tracing::error!(%publish, "Failed to send response");
            }
        }
    }
}

async fn handle_one_request<R>(
    ctx: &R::Context,
    request: &nats::service::Request,
) -> json::Result<bytes::Bytes>
where
    R: JsonRpc2Service,
{
    let jsonrpc::Request { params, id, .. } = json::from_slice(&request.message.payload)?;
    let request = params
        .map(json::from_value::<<R as JsonRpc2>::Request>)
        .transpose()?;
    tracing::debug!(?request);
    let result = R::serve(ctx, request).await;
    let response = jsonrpc::Response::from_result(id, result)?;
    let bytes = json::to_vec(&response)?;
    Ok(bytes.into())
}

fn nats_service_error(error: json::Error) -> nats::service::error::Error {
    nats::service::error::Error {
        status: error.to_string(),
        code: usize::MAX,
    }
}
