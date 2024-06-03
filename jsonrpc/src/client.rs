use super::*;

/// Generic JSONRPC Client object that can be used with different transports `T`
///
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
    T::Error: From<json::Error> + Send + Sync,
{
    /// Create new `AsyncClient` instance with a given `transport`.
    ///
    pub fn with_transport(transport: T) -> Self {
        let id = AtomicU64::new(0);
        Self { transport, id }
    }

    /// Execute a JSONRPC call, desribed by `R as JsonRpc2`
    /// # Errors
    /// The outer error is the transport error
    /// The inner error is the error object defined by the `R as JsonRpc2` call itself
    ///
    pub async fn call<R>(
        &self,
        request: impl Into<Option<R::Request>> + Send,
    ) -> Result<Result<R::Response, R::Error>, T::Error>
    where
        R: JsonRpc2,
    {
        let id = self.id().into();
        let request = Request::from_request::<R>(id, request.into())?;
        let response = self.transport.call(request).await?;
        let response = response
            .into_typed_result::<R>()?
            .inspect(|response| tracing::trace!(?response))
            .inspect_err(|error| tracing::trace!(?error));

        Ok(response)
    }
}
