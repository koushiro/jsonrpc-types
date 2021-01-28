use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::{Error, ErrorCode},
    id::Id,
    v2::version::Version,
};

/// Represents JSON-RPC 2.0 success response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Success {
    /// A String specifying the version of the JSON-RPC protocol.
    pub jsonrpc: Version,
    /// Successful execution result.
    pub result: Value,
    /// Correlation id.
    ///
    /// It **MUST** be the same as the value of the id member in the Request Object.
    pub id: Id,
}

impl fmt::Display for Success {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).expect("`Success` is serializable");
        write!(f, "{}", json)
    }
}

impl Success {
    /// Creates a JSON-RPC 2.0 success response.
    pub fn new(result: Value, id: Id) -> Self {
        Self {
            jsonrpc: Version::V2_0,
            result,
            id,
        }
    }
}

/// Represents JSON-RPC 2.0 failure response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Failure {
    /// A String specifying the version of the JSON-RPC protocol.
    pub jsonrpc: Version,
    /// Failed execution error.
    pub error: Error,
    /// Correlation id.
    ///
    /// It **MUST** be the same as the value of the id member in the Request Object.
    pub id: Id,
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).expect("`Failure` is serializable");
        write!(f, "{}", json)
    }
}

impl Failure {
    /// Creates a JSON-RPC 2.0 failure response.
    pub fn new(error: Error, id: Id) -> Self {
        Self {
            jsonrpc: Version::V2_0,
            error,
            id,
        }
    }
}

/// Represents success / failure output of response.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Output {
    /// Success response output
    Success(Success),
    /// Failure response output
    Failure(Failure),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).expect("`Output` is serializable");
        write!(f, "{}", json)
    }
}

impl Output {
    /// Creates a new output with given  `Version`, `Result` and `Id`.
    pub fn new(result: Result<Value, Error>, id: Id) -> Self {
        match result {
            Ok(result) => Output::Success(Success::new(result, id)),
            Err(error) => Output::Failure(Failure::new(error, id)),
        }
    }

    /// Creates a new failure output indicating malformed request.
    pub fn invalid_request(id: Id) -> Self {
        Output::Failure(Failure::new(Error::new(ErrorCode::InvalidRequest), id))
    }

    /// Gets the JSON-RPC protocol version.
    pub fn version(&self) -> Version {
        match self {
            Output::Success(s) => s.jsonrpc,
            Output::Failure(f) => f.jsonrpc,
        }
    }

    /// Gets the correlation id.
    pub fn id(&self) -> Id {
        match self {
            Output::Success(s) => s.id.clone(),
            Output::Failure(f) => f.id.clone(),
        }
    }
}

impl From<Output> for Result<Value, Error> {
    // Convert into a result.
    // Will be `Ok` if it is a `SuccessResponse` and `Err` if `FailureResponse`.
    fn from(output: Output) -> Result<Value, Error> {
        match output {
            Output::Success(s) => Ok(s.result),
            Output::Failure(f) => Err(f.error),
        }
    }
}

/// JSON-RPC Response object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Response {
    /// Single response
    Single(Output),
    /// Response to batch request (batch of responses)
    Batch(Vec<Output>),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).expect("`Response` is serializable");
        write!(f, "{}", json)
    }
}

impl From<Success> for Response {
    fn from(success: Success) -> Self {
        Response::Single(Output::Success(success))
    }
}

impl From<Failure> for Response {
    fn from(failure: Failure) -> Self {
        Response::Single(Output::Failure(failure))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn success_response_cases() -> Vec<(Success, &'static str)> {
        vec![(
            // JSON-RPC 2.0 success response
            Success {
                jsonrpc: Version::V2_0,
                result: Value::Bool(true),
                id: Id::Num(1),
            },
            r#"{"jsonrpc":"2.0","result":true,"id":1}"#,
        )]
    }

    fn failure_response_cases() -> Vec<(Failure, &'static str)> {
        vec![(
            // JSON-RPC 2.0 failure response
            Failure {
                jsonrpc: Version::V2_0,
                error: Error::parse_error(),
                id: Id::Num(1),
            },
            r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#,
        )]
    }

    #[test]
    fn success_response_serialization() {
        for (success_response, expect) in success_response_cases() {
            let ser = serde_json::to_string(&success_response).unwrap();
            assert_eq!(ser, expect);
            let de = serde_json::from_str::<Success>(expect).unwrap();
            assert_eq!(de, success_response);
        }
    }

    #[test]
    fn failure_response_serialization() {
        for (failure_response, expect) in failure_response_cases() {
            let ser = serde_json::to_string(&failure_response).unwrap();
            assert_eq!(ser, expect);
            let de = serde_json::from_str::<Failure>(expect).unwrap();
            assert_eq!(de, failure_response);
        }
    }

    #[test]
    fn response_output_serialization() {
        for (success_response, expect) in success_response_cases() {
            let response_output = Output::Success(success_response);
            assert_eq!(serde_json::to_string(&response_output).unwrap(), expect);
            assert_eq!(
                serde_json::from_str::<Output>(expect).unwrap(),
                response_output
            );
        }

        for (failure_response, expect) in failure_response_cases() {
            let response_output = Output::Failure(failure_response);
            assert_eq!(serde_json::to_string(&response_output).unwrap(), expect);
            assert_eq!(
                serde_json::from_str::<Output>(expect).unwrap(),
                response_output
            );
        }
    }

    #[test]
    fn response_serialization() {
        for (success_resp, expect) in success_response_cases() {
            let success_response = Response::Single(Output::Success(success_resp.clone()));
            assert_eq!(serde_json::to_string(&success_response).unwrap(), expect);
            assert_eq!(
                serde_json::from_str::<Response>(expect).unwrap(),
                success_response
            );
        }

        for (failure_resp, expect) in failure_response_cases() {
            let failure_response = Response::Single(Output::Failure(failure_resp.clone()));
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
                Output::Success(success_resp),
                Output::Failure(failure_resp),
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
    fn invalid_response() {
        let cases = vec![
            // JSON-RPC 2.0 invalid response
            r#"{"jsonrpc":"2.0","result":true,"id":1,"unknown":[]}"#,
            r#"{"jsonrpc":"2.0","error":{"code": -32700,"message": "Parse error"},"id":1,"unknown":[]}"#,
            r#"{"jsonrpc":"2.0","result":true,"error":{"code": -32700,"message": "Parse error"},"id":1}"#,
            r#"{"jsonrpc":"2.0","id":1}"#,
            r#"{"jsonrpc":"2.0","unknown":[]}"#,
        ];

        for case in cases {
            let response = serde_json::from_str::<Response>(case);
            assert!(response.is_err());
        }
    }

    #[test]
    fn valid_response() {
        let cases = vec![
            // JSON-RPC 2.0 valid response
            r#"{"jsonrpc":"2.0","result":true,"id":1}"#,
            r#"{"jsonrpc":"2.0","error":{"code": -32700,"message": "Parse error"},"id":1}"#,
        ];

        for case in cases {
            let response = serde_json::from_str::<Response>(case);
            assert!(response.is_ok());
        }
    }
}
