use std::collections::BTreeMap;
use std::fmt;
use std::mem;
use std::sync::Arc;

use futures::future;
use futures::FutureExt;

use super::*;

#[derive(Default)]
pub(crate) struct Endpoints {
    endpoints: BTreeMap<&'static str, future::BoxFuture<'static, ()>>,
}

impl Endpoints {
    pub(crate) fn endpoint<R>(mut self, ctx: R, endpoint: Endpoint) -> Self
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
        let ctx = Arc::new(ctx);
        let ep = endpoint
            .for_each(move |request| {
                let ctx = ctx.clone();
                handle_request(ctx, request)
            })
            .boxed();
        self.endpoints.insert(R::METHOD, ep);
        self
    }

    pub(crate) fn spawn_on(
        &mut self,
        tasks: &mut tokio::task::JoinSet<anyhow::Result<()>>,
    ) -> Vec<tokio::task::AbortHandle> {
        mem::take(&mut self.endpoints)
            .into_iter()
            .map(|(method, handler)| {
                tracing::info!(method, "spawning handler for");
                let task = async {
                    handler.await;
                    Ok(())
                };
                tasks.spawn(task)
            })
            .collect::<Vec<_>>()
    }
}

impl fmt::Debug for Endpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Endpoints")
            .field("endpoints", &"<BTreeMap<R::METHOD, Lazy>>")
            .finish()
    }
}

async fn handle_request<R>(ctx: Arc<R>, request: nats::service::Request)
where
    R: JsonRpc2
        + JsonRpc2Service<
            <R as JsonRpc2>::Request,
            Response = <R as JsonRpc2>::Response,
            Error = <R as JsonRpc2>::Error,
        >,
{
    handle_nats_request(ctx.as_ref(), request)
        .await
        .unwrap_or_else(|error| tracing::error!(%error, "Failed to send response"))
}
