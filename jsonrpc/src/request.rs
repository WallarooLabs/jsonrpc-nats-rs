use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub jsonrpc: JsonRpc2Version,
    pub id: json::Value,
    pub method: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<json::Value>,
}

impl Request {
    pub fn new(method: &'static str, params: Option<json::Value>, id: json::Value) -> Self {
        let jsonrpc = JsonRpc2Version::JsonRpc2;
        let method = Cow::from(method);

        Self {
            jsonrpc,
            method,
            params,
            id,
        }
    }

    pub fn from_request<R>(
        id: json::Value,
        request: Option<R::Request>,
    ) -> Result<Self, json::Error>
    where
        R: JsonRpc2,
    {
        request
            .map(json::to_value)
            .transpose()
            .map(|params| Self::new(R::METHOD, params, id))
    }

    pub fn into_request<R>(self) -> json::Result<(json::Value, R::Request)>
    where
        R: JsonRpc2,
    {
        let Self { id, params, .. } = self;
        let value = params.unwrap_or_default();
        json::from_value(value).map(|request| (id, request))
    }
}
