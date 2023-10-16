use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    jsonrpc: JsonRpc2Version,
    id: json::Value,
    #[serde(flatten)]
    payload: Payload,
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

    pub fn success(result: json::Value, id: json::Value) -> Self {
        let jsonrpc = JsonRpc2Version::JsonRpc2;
        let payload = Payload::success(result);
        Self {
            jsonrpc,
            id,
            payload,
        }
    }

    pub fn failure(error: ErrorObject, id: json::Value) -> Self {
        let jsonrpc = JsonRpc2Version::JsonRpc2;
        let payload = Payload::failure(error);
        Self {
            jsonrpc,
            id,
            payload,
        }
    }

    pub fn from_json_error(error: json::Error, id: json::Value) -> Self {
        Self::failure(ErrorObject::parse_error(error), id)
    }

    pub fn failure2<T>(error: Error<T>, id: json::Value) -> Self
    where
        T: StdError + 'static,
    {
        Self::failure(ErrorObject::from(error), id)
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
            Ok(success) => Self::serialize_success(success, id),
            Err(failure) => Self::serialize_failure(failure, id),
        }
    }

    pub fn serialize_success<T>(success: T, id: json::Value) -> json::Result<Self>
    where
        T: Serialize,
    {
        json::to_value(success).map(|result| Self::success(result, id))
    }

    pub fn serialize_failure<E>(failure: E, id: json::Value) -> json::Result<Self>
    where
        E: Serialize,
    {
        json::to_value(failure)
            .map(ErrorObject::with_data)
            .map(|error| Self::failure(error, id))
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
