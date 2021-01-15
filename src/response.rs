use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub result: Value,
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

/// Represents success / failure output of response.
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
    pub fn new(jsonrpc: Option<Version>, id: Id, result: Result<Value, Error>) -> Self {
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

impl From<ResponseOutput> for Result<Value, Error> {
    // Convert into a result.
    // Will be `Ok` if it is a `SuccessResponse` and `Err` if `FailureResponse`.
    fn from(output: ResponseOutput) -> Result<Value, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn success_response() -> (SuccessResponse, &'static str) {
        (
            SuccessResponse {
                jsonrpc: Some(Version::V2),
                result: Value::Bool(true),
                id: Id::Num(1),
            },
            r#"{"jsonrpc":"2.0","result":true,"id":1}"#,
        )
    }

    fn failure_response() -> (FailureResponse, &'static str) {
        (
            FailureResponse {
                jsonrpc: Some(Version::V2),
                error: Error::parse_error(),
                id: Id::Num(1),
            },
            r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#,
        )
    }

    #[test]
    fn success_response_serialization() {
        let (success_response, expect) = success_response();
        let ser = serde_json::to_string(&success_response).unwrap();
        assert_eq!(ser, expect);
        let de = serde_json::from_str::<SuccessResponse>(expect).unwrap();
        assert_eq!(de, success_response);
    }

    #[test]
    fn failure_response_serialization() {
        let (failure_response, expect) = failure_response();
        let ser = serde_json::to_string(&failure_response).unwrap();
        assert_eq!(ser, expect);
        let de = serde_json::from_str::<FailureResponse>(expect).unwrap();
        assert_eq!(de, failure_response);
    }

    #[test]
    fn response_output_serialization() {
        let (success_response, expect) = success_response();
        let response_output = ResponseOutput::Success(success_response);
        assert_eq!(serde_json::to_string(&response_output).unwrap(), expect);
        assert_eq!(
            serde_json::from_str::<ResponseOutput>(expect).unwrap(),
            response_output
        );

        let (failure_response, expect) = failure_response();
        let response_output = ResponseOutput::Failure(failure_response);
        assert_eq!(serde_json::to_string(&response_output).unwrap(), expect);
        assert_eq!(
            serde_json::from_str::<ResponseOutput>(expect).unwrap(),
            response_output
        );
    }

    #[test]
    fn response_serialization() {
        let (success_resp, success_expect) = success_response();
        let success_response = Response::Single(ResponseOutput::Success(success_resp.clone()));
        assert_eq!(
            serde_json::to_string(&success_response).unwrap(),
            success_expect
        );
        assert_eq!(
            serde_json::from_str::<Response>(success_expect).unwrap(),
            success_response
        );

        let (failure_resp, failure_expect) = failure_response();
        let failure_response = Response::Single(ResponseOutput::Failure(failure_resp.clone()));
        assert_eq!(
            serde_json::to_string(&failure_response).unwrap(),
            failure_expect
        );
        assert_eq!(
            serde_json::from_str::<Response>(failure_expect).unwrap(),
            failure_response
        );

        let batch_response = Response::Batch(vec![
            ResponseOutput::Success(success_resp),
            ResponseOutput::Failure(failure_resp),
        ]);
        let batch_expect = format!("[{},{}]", success_expect, failure_expect);
        assert_eq!(
            serde_json::to_string(&batch_response).unwrap(),
            batch_expect
        );
        assert_eq!(
            serde_json::from_str::<Response>(&batch_expect).unwrap(),
            batch_response
        );
    }

    #[test]
    fn invalid_response() {
        let cases = vec![
            r#"{
                "jsonrpc":"2.0",
                "result":true,
                "id":1,
                "unknown":[]
            }"#,
            r#"{
                "jsonrpc":"2.0",
                "error":{
                    "code": -32700,
                    "message": "Parse error"
                },
                "id":1,
                "unknown":[]
            }"#,
            r#"{
                "jsonrpc":"2.0",
                "result":true,
                "error":{
                    "code": -32700,
                    "message": "Parse error"
                },
                "id":1
            }"#,
            r#"{"unknown":[]}"#,
        ];

        for case in cases {
            let response = serde_json::from_str::<Response>(case);
            assert!(response.is_err());
        }
    }
}
