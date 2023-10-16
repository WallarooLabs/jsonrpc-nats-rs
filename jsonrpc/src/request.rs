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
    pub fn new(method: &'static str, params: Option<json::Value>, id: u64) -> Self {
        let jsonrpc = JsonRpc2Version::JsonRpc2;
        let method = Cow::from(method);
        let id = json::Value::from(id);

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
        let jsonrpc = JsonRpc2Version::JsonRpc2;
        let method = R::METHOD.into();
        let params = request.map(json::to_value).transpose()?;

        Ok(Self {
            jsonrpc,
            method,
            params,
            id,
        })
    }

    pub fn into_request<R>(self) -> json::Result<Option<R::Request>>
    where
        R: JsonRpc2,
    {
        self.params.map(json::from_value).transpose()
    }
}
