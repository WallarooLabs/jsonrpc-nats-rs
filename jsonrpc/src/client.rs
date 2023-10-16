use super::*;

#[derive(Debug)]
pub struct AsyncClient<T> {
    transport: T,
    id: AtomicU64,
}

impl<T> AsyncClient<T> {
    fn id(&self) -> u64 {
        self.id.fetch_add(1, Ordering::SeqCst)
    }
}

impl<T> AsyncClient<T>
where
    T: service::JsonRpc2Service<Request, Response = Response>,
    T::Error: fmt::Debug,
{
    pub fn with_transport(transport: T) -> Self {
        let id = AtomicU64::new(0);
        Self { transport, id }
    }

    pub async fn call<R>(&self, request: R::Request) -> Result<R::Response, R::Error>
    where
        R: JsonRpc2,
    {
        let id = self.id().into();
        let request = Request::from_request::<R>(id, Some(request))
            .expect("Failed to convert to JSONRPC REQUEST");
        let response = self
            .transport
            .call(request)
            .await
            .expect("Transport failed");

        response
            .into_typed_result::<R>()
            .expect("Failed to convert from JSOM RESPONSE")
            .tap_ok(|response| tracing::trace!(?response))
            .tap_err(|error| tracing::trace!(?error))
    }
}
