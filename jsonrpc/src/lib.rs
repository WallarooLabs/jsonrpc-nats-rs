use std::borrow::Cow;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{de, Deserialize, Serialize};
use serde_json as json;
use thiserror::Error;

pub use jsonrpc_derive::JsonRpc2;

pub use self::client::AsyncClient;
pub use self::error::ErrorObject;
pub use self::request::Request;
pub use self::response::Payload;
pub use self::response::Response;
pub use self::service::handle_jsonrpc_call;
pub use self::service::JsonRpc2Service;

mod client;
mod service;

mod error;
mod request;
mod response;

/// Define a single JSONRPC function
pub trait JsonRpc2: Send + Sync {
    /// JSONRPC method name
    const METHOD: &'static str;
    /// Shape of the request
    type Request: fmt::Debug + Serialize + de::DeserializeOwned + Send + Sync;
    /// Shape of the response
    type Response: fmt::Debug + Serialize + de::DeserializeOwned + Send + Sync;
    /// Shape of the error
    type Error: fmt::Debug + Serialize + de::DeserializeOwned + Send + Sync;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JsonRpc2Version {
    #[serde(rename = "2.0")]
    JsonRpc2,
}
