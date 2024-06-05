use std::borrow::Cow;
use std::collections::HashMap;
use std::mem;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures::Stream;
use serde_json as json;
use tokio::sync::Mutex;

use super::*;

pub use self::endpoint::Endpoint;

mod endpoint;

#[derive(Debug)]
pub struct Service {
    rx: Mutex<Option<mpsc::Receiver<Request>>>,
    endpoints: Mutex<HashMap<&'static str, mpsc::Sender<Request>>>,
}

impl Service {
    pub(super) fn new(rx: mpsc::Receiver<Request>) -> Self {
        let rx = Mutex::new(Some(rx));
        let endpoints = Mutex::new(HashMap::new());
        Self { rx, endpoints }
    }

    pub(super) async fn endpoint(&self, method: &'static str) -> Endpoint {
        let (tx, rx) = mpsc::channel(10);
        self.endpoints.lock().await.insert(method, tx);
        Endpoint::new(method, rx)
        // match &self.inner {
        //     ServiceInner::Preparing { endpoints, .. } => {
        //         let (tx, rx) = mpsc::channel(10);
        //         endpoints.lock().await.insert(method, tx);
        //         Endpoint::new(method, rx)
        //     }
        //     &ServiceInner::Empty | ServiceInner::Running { .. } => {
        //         panic!("Cannot call endpoint while running")
        //     }
        // }
    }

    pub(super) async fn spawn_on(&self, tasks: &mut tokio::task::JoinSet<anyhow::Result<()>>) {
        use std::ops::DerefMut;
        if let Some(rx) = self.rx.lock().await.take() {
            let mut endpoints = self.endpoints.lock().await;
            let endpoints = mem::take(endpoints.deref_mut());
            tasks.spawn(run_service(rx, endpoints));
        } else {
            panic!("Service already running");
        }
    }
}

async fn run_service(
    mut rx: mpsc::Receiver<Request>,
    endpoints: HashMap<&'static str, mpsc::Sender<Request>>,
) -> anyhow::Result<()> {
    while let Some(mut request) = rx.recv().await {
        let (method, id) = match method_name_and_id(&request) {
            Ok((method, id)) => (method, id),
            Err(response) => {
                let response = json::to_value(response)
                    .expect("jsonrpc::Response conversion to json::Value failed");
                request
                    .respond(response)
                    .await
                    .unwrap_or_else(|err| tracing::error!(?err, "JSON error"));
                continue;
            }
        };

        let tx = match endpoints.get(method.as_ref()) {
            Some(tx) => tx,
            None => {
                let response = jsonrpc::Response::method_not_found(id, method.as_ref());
                let response = json::to_value(response)
                    .expect("jsonrpc::Response conversion to json::Value failed");
                request
                    .respond(response)
                    .await
                    .unwrap_or_else(|err| tracing::error!(?err, "method not found"));
                continue;
            }
        };

        if let Err(mpsc::error::SendError(mut request)) = tx.send(request).await {
            let error = jsonrpc::ErrorObject::internal_error("failed to send");
            let response = jsonrpc::Response::failure(id, error);
            let response = json::to_value(response)
                .expect("jsonrpc::Response conversion to json::Value failed");
            request
                .respond(response)
                .await
                .unwrap_or_else(|err| tracing::error!(?err, "Internal error"));
        }
    }
    Ok(())
}

fn method_name_and_id(
    request: &Request,
) -> Result<(Cow<'static, str>, json::Value), jsonrpc::Response> {
    json::from_value::<jsonrpc::Request>(request.message.clone())
        .map(|request| (request.method, request.id))
        .map_err(|error| jsonrpc::Response::from_json_error(json::Value::Null, error))
}
