use super::*;

#[derive(Debug)]
pub struct AsyncServer<T> {
    transport: T,
}

impl<T> AsyncServer<T>
where
    T: ServerTransport + fmt::Debug + 'static,
{
    pub async fn serve<R>(&mut self) -> Result<(), ()>
    where
        R: JsonRpc2,
    {
        tracing::trace!(?self.transport);
        Ok(())
    }
}
