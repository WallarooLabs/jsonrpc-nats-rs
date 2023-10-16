// use std::error::Error as StdError;

use async_nats as nats;
use bytes::Bytes;
use jsonrpc::async_trait;
use jsonrpc::AsyncClient;
use jsonrpc::JsonRpc2;
use jsonrpc::JsonRpc2Service;
use serde_json as json;

pub use self::client::Client;
pub use self::server::Server;

mod client;
mod server;
