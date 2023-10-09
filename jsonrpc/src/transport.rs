use super::*;

#[async_trait]
pub trait ClientTransport {
    type TransportError: StdError;
    type ResponseHandle;

    async fn send_request(
        &mut self,
        request: &Request,
    ) -> Result<Self::ResponseHandle, Error<Self::TransportError>>;

    async fn recv_response(
        &mut self,
        handle: Self::ResponseHandle,
    ) -> Result<Response, Error<Self::TransportError>>;
}

#[async_trait]
pub trait ServerTransport {
    type TransportError: StdError;

    async fn recv_request(&mut self) -> Result<Request, Error<Self::TransportError>>;

    async fn send_response(
        &mut self,
        response: Response,
    ) -> Result<(), Error<Self::TransportError>>;
}
