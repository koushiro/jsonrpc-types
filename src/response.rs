use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::error::{Error, ErrorCode};
use crate::id::Id;
use crate::version::Version;

/// Represents successful JSON-RPC response.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SuccessResponse {
    /// A String specifying the version of the JSON-RPC protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonrpc: Option<Version>,
    /// Successful execution result.
    pub result: JsonValue,
    /// Correlation id.
    ///
    /// It **MUST** be the same as the value of the id member in the Request Object.
    pub id: Id,
}

/// Represents failed JSON-RPC response.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FailureResponse {
    /// A String specifying the version of the JSON-RPC protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonrpc: Option<Version>,
    /// Failed execution error.
    pub error: Error,
    /// Correlation id.
    ///
    /// It **MUST** be the same as the value of the id member in the Request Object.
    pub id: Id,
}

/// Represents output of response - success or failure
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum ResponseOutput {
    /// Success response output
    Success(SuccessResponse),
    /// Failure response output
    Failure(FailureResponse),
}

impl ResponseOutput {
    /// Creates a new output with given  `Version`, `Id` and `Result`.
    pub fn new(jsonrpc: Option<Version>, id: Id, result: Result<JsonValue, Error>) -> Self {
        match result {
            Ok(result) => ResponseOutput::Success(SuccessResponse {
                jsonrpc,
                result,
                id,
            }),
            Err(error) => ResponseOutput::Failure(FailureResponse { jsonrpc, error, id }),
        }
    }

    /// Creates a new failure output indicating malformed request.
    pub fn invalid_request(jsonrpc: Option<Version>, id: Id) -> Self {
        ResponseOutput::Failure(FailureResponse {
            jsonrpc,
            id,
            error: Error::new(ErrorCode::InvalidRequest),
        })
    }

    /// Gets the JSON-RPC protocol version.
    pub fn version(&self) -> Option<Version> {
        match self {
            ResponseOutput::Success(s) => s.jsonrpc,
            ResponseOutput::Failure(f) => f.jsonrpc,
        }
    }

    /// Gets the correlation id.
    pub fn id(&self) -> Id {
        match self {
            ResponseOutput::Success(s) => s.id.clone(),
            ResponseOutput::Failure(f) => f.id.clone(),
        }
    }
}

impl From<ResponseOutput> for Result<JsonValue, Error> {
    /// Convert into a result. Will be `Ok` if it is a `Success` and `Err` if `Failure`.
    fn from(output: ResponseOutput) -> Result<JsonValue, Error> {
        match output {
            ResponseOutput::Success(s) => Ok(s.result),
            ResponseOutput::Failure(f) => Err(f.error),
        }
    }
}

/// JSON-RPC Response object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Response {
    /// Single response
    Single(ResponseOutput),
    /// Response to batch request (batch of responses)
    Batch(Vec<ResponseOutput>),
}

impl From<SuccessResponse> for Response {
    fn from(success: SuccessResponse) -> Self {
        Response::Single(ResponseOutput::Success(success))
    }
}

impl From<FailureResponse> for Response {
    fn from(failure: FailureResponse) -> Self {
        Response::Single(ResponseOutput::Failure(failure))
    }
}
