use super::*;

#[derive(Debug)]
pub struct Endpoint {
    method: &'static str,
    rx: mpsc::Receiver<Request>,
}

impl Endpoint {
    pub(super) fn new(method: &'static str, rx: mpsc::Receiver<Request>) -> Self {
        Self { method, rx }
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn method(&self) -> &str {
        self.method
    }
}

impl Stream for Endpoint {
    type Item = Request;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}
