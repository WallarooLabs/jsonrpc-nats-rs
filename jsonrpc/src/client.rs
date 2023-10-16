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
    }
}

impl<T> AsyncClient<T>
where
    T: ClientTransport + fmt::Debug + 'static,
{
    pub fn with_transport_deprecated(transport: T) -> Self {
        let id = AtomicU64::new(0);
        Self { transport, id }
    }

    /// Makes JSON RPC call with given request and returns the recevied response
    /// # Errors
    /// Error could either transport, JSON serialization, or the error returned
    /// by the RP call itself.
    #[tracing::instrument]
    pub async fn call_deprecated<R>(
        &mut self,
        request: Option<<R as JsonRpc2>::Request>,
    ) -> Result<<R as JsonRpc2>::Response, Error<<T as ClientTransport>::TransportError>>
    where
        R: JsonRpc2Client,
    {
        let id = self.id().into();

        let request = Request::from_request::<R>(id, request)
            .tap_err(|e| tracing::error!(%e, "Request::from_input"))?;

        let handle = self.transport.send_request(&request).await?;
        let result = self
            .transport
            .recv_response(handle)
            .await?
            .into_result()
            .tap_ok(|v| tracing::trace!(%v, "recv_response"))?;

        let response = json::from_value(result)
            .tap_err(|e| tracing::trace!(%e, "Failed to parse response"))?;

        Ok(response)
    }
}
