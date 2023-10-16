use tower::Service;

use jsonrpc::Request;
// use jsonrpc::Response;

#[derive(Debug)]
pub struct JsonRpcClient<T> {
    transport: T,
}

impl<T> JsonRpcClient<T>
where
    T: Service<Request>,
{
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn transport(&self) -> &T {
        &self.transport
    }
}
