use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{de, Deserialize, Serialize};
use serde_json as json;
use tap::TapFallible;
use thiserror::Error;

pub use async_trait::async_trait;

pub use self::client::AsyncClient;
pub use self::error::Error;
pub use self::error::ErrorObject;
pub use self::error::TransportError;
pub use self::request::Request;
pub use self::response::Response;
pub use self::service::JsonRpc2Service;
pub use self::transport::ServerTransport;

pub mod client;
pub mod server;
mod service;

mod error;
mod request;
mod response;
mod transport;

pub trait JsonRpc2 {
    const METHOD: &'static str;
    type Request: fmt::Debug + Serialize + de::DeserializeOwned;
    type Response: fmt::Debug + Serialize + de::DeserializeOwned;
    type Error: fmt::Debug + Serialize + de::DeserializeOwned;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JsonRpc2Version {
    #[serde(rename = "2.0")]
    JsonRpc2,
}
