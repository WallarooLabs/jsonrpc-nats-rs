# Async Rust JSONRPC framework

The implementation is modeled on `tower::Service`, but does not use it directly.
The reason is that `tower::Service` is not `async` and hence is less ergonomic
than it could be otherwise.

NOTE: There is `tower-async`, but it requires nightly compiler.

## (VHLD) Very High Level Description

```Rust
pub trait JsonRpc2 {
    const METHOD: &'static str;
    type Request: fmt::Debug + Serialize + de::DeserializeOwned;
    type Response: fmt::Debug + Serialize + de::DeserializeOwned;
    type Error: fmt::Debug + Serialize + de::DeserializeOwned;
}

pub trait JsonRpc2Service<Request> {
    type Response;
    type Error;
    async fn call(&self, request: Request) -> Result<Self::Response, Self::Error>;
}

```

These two traits are what it takes for a consumer to implement the RPC
server and client.

See `pingpong` crate for an example of implementing it.

## Derive JsonRpc2

```Rust
#[derive(Debug, JsonRpc2)]
#[jsonrpc(
    method = "frob",
    request = "FrobRequest",
    response = "FrobResponse",
    error = "FrobError",
    crates(jsonrpc = "my_jsonrpc", serde_json = "json"),
    client = "FrobClient",
)]
struct Frob;
```

### Attributes

`method` - String, Required. JSONRPC method name.

`request` - Struct name, Optional. The name of the request object struct. Defaults to `XxxRequest`, where `Xxx` is derive target.

`response` - Struct name, Optional. The name of the response object struct. Defaults to `XxxResponse`, where `Xxx` is derive target.

`error` - Struct name, Optional.The name os the error object struct. Defaults to `XxxError`, where `Xxx` is derive target.

`crates` - Attribute list, Optional. When `jsonrpc` and/or `serde_json` crates are renamed, specifies their new names.

`client` - Client extension trait generation attribute. Optional.
  If not present, client extension trait is not generated.
  If present without value - client extension trait is generated for `jsonrpc::AsyncClient<T>`
  If present with value then client extension trait is generated for the newtype struct with the given name as well as helper traits and client extension trait

```Rust
pub struct FrobClient(pub jsonrpc::AsyncClient<T>);

impl<T> From<jsonrpc::AsyncClient<T>> for FrobClient<T> {
    fn from(client: jsonrpc::AsyncClient<T>) -> Self {
        Self(client)
    }
}

impl<T> Deref for FrobClient<T> {
    type Target = jsonrpc::AsyncClient<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
