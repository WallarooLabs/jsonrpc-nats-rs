use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub jsonrpc: JsonRpc2Version,
    pub id: json::Value,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Success { result: json::Value },
    Failure { error: ErrorObject },
}

impl Response {
    pub fn id(&self) -> &json::Value {
        &self.id
    }

    pub fn method_not_found(id: json::Value, method: &str) -> Self {
        let error = ErrorObject::method_not_found(method);
        Self::failure(id, error)
    }

    pub fn success(id: json::Value, result: json::Value) -> Self {
        let jsonrpc = JsonRpc2Version::JsonRpc2;
        let payload = Payload::success(result);
        Self {
            jsonrpc,
            id,
            payload,
        }
    }

    pub fn failure(id: json::Value, error: ErrorObject) -> Self {
        let jsonrpc = JsonRpc2Version::JsonRpc2;
        let payload = Payload::failure(error);
        Self {
            jsonrpc,
            id,
            payload,
        }
    }

    pub fn from_json_error(id: json::Value, error: json::Error) -> Self {
        Self::failure(id, ErrorObject::parse_error(error))
    }

    pub fn into_result(self) -> Result<json::Value, ErrorObject> {
        self.payload.into_result()
    }

    pub fn into_typed_result<R>(self) -> json::Result<Result<R::Response, R::Error>>
    where
        R: JsonRpc2,
    {
        self.payload.into_typed_result::<R>()
    }

    pub fn from_result<T, E>(id: json::Value, result: Result<T, E>) -> json::Result<Self>
    where
        T: Serialize,
        E: Serialize,
    {
        match result {
            Ok(success) => Self::serialize_success(id, success),
            Err(failure) => Self::serialize_failure(id, failure),
        }
    }

    pub fn serialize_success<T>(id: json::Value, success: T) -> json::Result<Self>
    where
        T: Serialize,
    {
        json::to_value(success).map(|result| Self::success(id, result))
    }

    pub fn serialize_failure<E>(id: json::Value, failure: E) -> json::Result<Self>
    where
        E: Serialize,
    {
        json::to_value(failure)
            .map(ErrorObject::with_data)
            .map(|error| Self::failure(id, error))
    }
}

impl Payload {
    pub fn success(result: json::Value) -> Self {
        Self::Success { result }
    }

    pub fn failure(error: ErrorObject) -> Self {
        Self::Failure { error }
    }

    pub fn into_result(self) -> Result<json::Value, ErrorObject> {
        match self {
            Self::Success { result } => Ok(result),
            Self::Failure { error } => Err(error),
        }
    }

    pub fn into_typed_result<R>(self) -> json::Result<Result<R::Response, R::Error>>
    where
        R: JsonRpc2,
    {
        match self {
            Self::Success { result } => json::from_value(result).map(Ok),
            Self::Failure { error } => error.extract_error().map(Err),
        }
    }
}
