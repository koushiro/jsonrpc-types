// For compatibility with JSON-RPC v1 specification.
#[cfg(feature = "v1-compat")]
mod compat;
mod error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::id::Id;
use crate::version::Version;

pub use self::error::{Error, ErrorCode};

/// Represents successful JSON-RPC response.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(not(feature = "v1-compat"), derive(Serialize, Deserialize))]
#[cfg_attr(not(feature = "v1-compat"), serde(deny_unknown_fields))]
pub struct SuccessResponse {
    /// A String specifying the version of the JSON-RPC protocol.
    pub jsonrpc: Option<Version>,
    /// Successful execution result.
    pub result: Value,
    /// Correlation id.
    ///
    /// It **MUST** be the same as the value of the id member in the Request Object.
    pub id: Id,
}

/// Represents failed JSON-RPC response.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(not(feature = "v1-compat"), derive(Serialize, Deserialize))]
#[cfg_attr(not(feature = "v1-compat"), serde(deny_unknown_fields))]
pub struct FailureResponse {
    /// A String specifying the version of the JSON-RPC protocol.
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
            error: Error::new(ErrorCode::InvalidRequest),
            id,
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

    #[cfg(not(feature = "v1-compat"))]
    fn success_response_cases() -> Vec<(SuccessResponse, &'static str)> {
        vec![(
            // JSON-RPC v2 success response
            SuccessResponse {
                jsonrpc: Some(Version::V2_0),
                result: Value::Bool(true),
                id: Id::Num(1),
            },
            r#"{"jsonrpc":"2.0","result":true,"id":1}"#,
        )]
    }

    #[cfg(feature = "v1-compat")]
    fn success_response_cases() -> Vec<(SuccessResponse, &'static str)> {
        vec![
            (
                // JSON-RPC v1 success response
                SuccessResponse {
                    jsonrpc: None,
                    result: Value::Bool(true),
                    id: Id::Num(1),
                },
                r#"{"result":true,"error":null,"id":1}"#,
            ),
            (
                // JSON-RPC v2 success response
                SuccessResponse {
                    jsonrpc: Some(Version::V2_0),
                    result: Value::Bool(true),
                    id: Id::Num(1),
                },
                r#"{"jsonrpc":"2.0","result":true,"id":1}"#,
            ),
        ]
    }

    #[cfg(not(feature = "v1-compat"))]
    fn failure_response_cases() -> Vec<(FailureResponse, &'static str)> {
        vec![(
            // JSON-RPC v2 failure response
            FailureResponse {
                jsonrpc: Some(Version::V2_0),
                error: Error::parse_error(),
                id: Id::Num(1),
            },
            r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#,
        )]
    }

    #[cfg(feature = "v1-compat")]
    fn failure_response_cases() -> Vec<(FailureResponse, &'static str)> {
        vec![
            (
                // JSON-RPC v1 failure response
                FailureResponse {
                    jsonrpc: None,
                    error: Error::parse_error(),
                    id: Id::Num(1),
                },
                r#"{"error":{"code":-32700,"message":"Parse error"},"result":null,"id":1}"#,
            ),
            (
                // JSON-RPC v2 failure response
                FailureResponse {
                    jsonrpc: Some(Version::V2_0),
                    error: Error::parse_error(),
                    id: Id::Num(1),
                },
                r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#,
            ),
        ]
    }

    #[test]
    fn success_response_serialization() {
        for (success_response, expect) in success_response_cases() {
            let ser = serde_json::to_string(&success_response).unwrap();
            assert_eq!(ser, expect);
            let de = serde_json::from_str::<SuccessResponse>(expect).unwrap();
            assert_eq!(de, success_response);
        }
    }

    #[test]
    fn failure_response_serialization() {
        for (failure_response, expect) in failure_response_cases() {
            let ser = serde_json::to_string(&failure_response).unwrap();
            assert_eq!(ser, expect);
            let de = serde_json::from_str::<FailureResponse>(expect).unwrap();
            assert_eq!(de, failure_response);
        }
    }

    #[test]
    fn response_output_serialization() {
        for (success_response, expect) in success_response_cases() {
            let response_output = ResponseOutput::Success(success_response);
            assert_eq!(serde_json::to_string(&response_output).unwrap(), expect);
            assert_eq!(
                serde_json::from_str::<ResponseOutput>(expect).unwrap(),
                response_output
            );
        }

        for (failure_response, expect) in failure_response_cases() {
            let response_output = ResponseOutput::Failure(failure_response);
            assert_eq!(serde_json::to_string(&response_output).unwrap(), expect);
            assert_eq!(
                serde_json::from_str::<ResponseOutput>(expect).unwrap(),
                response_output
            );
        }
    }

    #[test]
    fn response_serialization() {
        for (success_resp, expect) in success_response_cases() {
            let success_response = Response::Single(ResponseOutput::Success(success_resp.clone()));
            assert_eq!(serde_json::to_string(&success_response).unwrap(), expect);
            assert_eq!(
                serde_json::from_str::<Response>(expect).unwrap(),
                success_response
            );
        }

        for (failure_resp, expect) in failure_response_cases() {
            let failure_response = Response::Single(ResponseOutput::Failure(failure_resp.clone()));
            assert_eq!(serde_json::to_string(&failure_response).unwrap(), expect);
            assert_eq!(
                serde_json::from_str::<Response>(expect).unwrap(),
                failure_response
            );
        }

        for ((success_resp, success_expect), (failure_resp, failure_expect)) in
            success_response_cases()
                .into_iter()
                .zip(failure_response_cases())
        {
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
    }

    #[test]
    fn invalid_response_v2() {
        let cases = vec![
            // JSON-RPC v2 invalid response
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
            r#"{"jsonrpc":"2.0","unknown":[]}"#,
        ];

        for case in cases {
            let response = serde_json::from_str::<Response>(case);
            assert!(response.is_err());
        }
    }

    #[test]
    #[cfg(feature = "v1-compat")]
    fn invalid_response_v1() {
        let cases = vec![
            // JSON-RPC v1 invalid response
            r#"{
                "result":true,
                "id":1,
                "unknown":[]
            }"#,
            r#"{
                "result":true,
                "error":{
                    "code": -32700,
                    "message": "Parse error"
                },
                "id":1
            }"#,
            r#"{
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
