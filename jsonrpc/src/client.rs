use super::*;

#[derive(Debug)]
pub struct AsyncClient<T> {
    transport: T,
    id: AtomicU64,
}

impl<T> AsyncClient<T>
where
    T: ClientTransport + fmt::Debug + 'static,
{
    pub fn with_transport(transport: T) -> Self {
        let id = AtomicU64::new(0);
        Self { transport, id }
    }

    fn id(&self) -> u64 {
        self.id.fetch_add(1, Ordering::SeqCst)
    }

    /// Makes JSON RPC call with given request and returns the recevied response
    /// # Errors
    /// Error could either transport, JSON serialization, or the error returned
    /// by the RP call itself.
    #[tracing::instrument]
    pub async fn call<R>(
        &mut self,
        request: Option<<R as JsonRpc2>::Request>,
    ) -> Result<<R as JsonRpc2>::Response, Error<<T as ClientTransport>::TransportError>>
    where
        R: JsonRpc2Client,
    {
        let id = self.id();

        let request = Request::from_input::<R>(id, request)
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
