use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Success {
        jsonrpc: JsonRpc2Version,
        result: json::Value,
        id: json::Value,
    },
    Failure {
        jsonrpc: JsonRpc2Version,
        error: ErrorObject,
        id: json::Value,
    },
}

impl Response {
    pub fn id(&self) -> &json::Value {
        match self {
            Self::Success { id, .. } | Self::Failure { id, .. } => id,
        }
    }

    pub fn success(result: json::Value, id: json::Value) -> Self {
        Self::Success {
            jsonrpc: JsonRpc2Version::JsonRpc2,
            result,
            id,
        }
    }

    pub fn failure(error: ErrorObject, id: json::Value) -> Self {
        Self::Failure {
            jsonrpc: JsonRpc2Version::JsonRpc2,
            error,
            id,
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
        match self {
            Self::Success { result, .. } => Ok(result),
            Self::Failure { error, .. } => Err(error),
        }
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
