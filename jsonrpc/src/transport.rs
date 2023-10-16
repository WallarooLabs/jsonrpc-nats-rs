use super::*;

#[async_trait]
pub trait ServerTransport {
    type TransportError: StdError;

    async fn recv_request(&mut self) -> Result<Request, Error<Self::TransportError>>;

    async fn send_response(
        &mut self,
        response: Response,
    ) -> Result<(), Error<Self::TransportError>>;
}
