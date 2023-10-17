use super::*;

#[derive(Debug, Serialize, Deserialize, Error)]
#[error("JSON-RPC Error response: code: {code}, message: \"{message}\", data: {data:?}")]
pub struct ErrorObject {
    pub code: i32,
    pub message: String,
    pub data: Option<json::Value>,
}

impl ErrorObject {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
    pub const SERVER_ERROR_LO: i32 = -32099;
    pub const SERVER_ERROR_HI: i32 = -32000;

    pub fn with_data(data: json::Value) -> Self {
        Self {
            code: 0,
            message: String::new(),
            data: Some(data),
        }
    }

    pub fn parse_error(error: json::Error) -> Self {
        Self {
            code: Self::PARSE_ERROR,
            message: error.to_string(),
            data: None,
        }
    }

    pub fn internal_error(text: impl ToString) -> Self {
        Self {
            code: Self::INTERNAL_ERROR,
            message: text.to_string(),
            data: None,
        }
    }

    pub fn is_parse_error(&self) -> bool {
        matches!(self.code, Self::PARSE_ERROR)
    }

    pub fn is_invalid_request(&self) -> bool {
        matches!(self.code, Self::INVALID_REQUEST)
    }

    pub fn is_method_not_found(&self) -> bool {
        matches!(self.code, Self::METHOD_NOT_FOUND)
    }

    pub fn is_invalid_params(&self) -> bool {
        matches!(self.code, Self::INVALID_PARAMS)
    }

    pub fn is_internal_error(&self) -> bool {
        matches!(self.code, Self::INTERNAL_ERROR)
    }

    pub fn is_server_error(&self) -> bool {
        matches!(self.code, Self::SERVER_ERROR_LO..=Self::SERVER_ERROR_HI)
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn extract_error<T>(self) -> Result<T, json::Error>
    where
        T: de::DeserializeOwned,
    {
        let value = self.data.unwrap_or_default();
        json::from_value(value)
    }
}
