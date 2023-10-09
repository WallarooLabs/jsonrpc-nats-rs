use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub jsonrpc: JsonRpc2Signature,
    pub method: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<json::Value>,
    pub id: json::Value,
}

impl Request {
    pub fn new(method: &'static str, params: Option<json::Value>, id: u64) -> Self {
        let jsonrpc = JsonRpc2Signature::JsonRpc2;
        let method = Cow::from(method);
        let id = json::Value::from(id);

        Self {
            jsonrpc,
            method,
            params,
            id,
        }
    }

    pub fn from_input<T>(id: u64, request: Option<T::Request>) -> Result<Self, json::Error>
    where
        T: JsonRpc2Client,
    {
        let method = T::METHOD;
        let params = T::jsonrpc2_params(request)?;
        let request = Self::new(method, params, id);

        Ok(request)
    }
}
