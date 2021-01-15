use std::fmt;
use std::marker::PhantomData;

use serde::{de, ser, Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Error, ErrorCode};
use crate::id::Id;
use crate::version::Version;

/// Represents successful JSON-RPC response.
#[derive(Debug, PartialEq, Clone)]
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

impl ser::Serialize for SuccessResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut state = ser::Serializer::serialize_struct(serializer, "SuccessResponse", 3)?;
        if self.jsonrpc.is_some() {
            ser::SerializeStruct::serialize_field(&mut state, "jsonrpc", &self.jsonrpc)?;
            ser::SerializeStruct::serialize_field(&mut state, "result", &self.result)?;
        } else {
            ser::SerializeStruct::serialize_field(&mut state, "result", &self.result)?;
            ser::SerializeStruct::serialize_field(&mut state, "error", &Option::<Error>::None)?;
        }
        ser::SerializeStruct::serialize_field(&mut state, "id", &self.id)?;
        ser::SerializeStruct::end(state)
    }
}

impl<'de> de::Deserialize<'de> for SuccessResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor<'de> {
            marker: PhantomData<SuccessResponse>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de> de::Visitor<'de> for Visitor<'de> {
            type Value = SuccessResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct SuccessResponse")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut jsonrpc = Option::<Version>::None;
                let mut result = Option::<Value>::None;
                let mut error = Option::<Option<Error>>::None;
                let mut id = Option::<Id>::None;

                while let Some(key) = de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Jsonrpc => {
                            if jsonrpc.is_some() {
                                return Err(de::Error::duplicate_field("jsonrpc"));
                            }
                            jsonrpc = Some(de::MapAccess::next_value::<Version>(&mut map)?)
                        }
                        Field::Result => {
                            if result.is_some() {
                                return Err(de::Error::duplicate_field("result"));
                            }
                            result = Some(de::MapAccess::next_value::<Value>(&mut map)?)
                        }
                        Field::Error => {
                            if error.is_some() {
                                return Err(de::Error::duplicate_field("error"));
                            }
                            error = Some(de::MapAccess::next_value::<Option<Error>>(&mut map)?)
                        }
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(de::MapAccess::next_value::<Id>(&mut map)?)
                        }
                    }
                }
                let (jsonrpc, result) = match (jsonrpc, result, error) {
                    (Some(version), Some(value), None) => (Some(version), value),
                    (None, Some(value), Some(error)) if error.is_none() => (None, value),
                    (_, None, _) => return Err(de::Error::missing_field("result")),
                    _ => {
                        return Err(de::Error::custom(
                            "Incompatible with JSON-RPC specification v1 and v2",
                        ));
                    }
                };
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                Ok(SuccessResponse {
                    jsonrpc,
                    result,
                    id,
                })
            }
        }

        de::Deserializer::deserialize_struct(
            deserializer,
            "SuccessResponse",
            FIELDS,
            Visitor {
                marker: PhantomData::<SuccessResponse>,
                lifetime: PhantomData,
            },
        )
    }
}

/// Represents failed JSON-RPC response.
#[derive(Debug, PartialEq, Clone)]
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

impl ser::Serialize for FailureResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut state = ser::Serializer::serialize_struct(serializer, "FailureResponse", 3)?;
        if self.jsonrpc.is_some() {
            ser::SerializeStruct::serialize_field(&mut state, "jsonrpc", &self.jsonrpc)?;
            ser::SerializeStruct::serialize_field(&mut state, "error", &self.error)?;
        } else {
            ser::SerializeStruct::serialize_field(&mut state, "error", &self.error)?;
            ser::SerializeStruct::serialize_field(&mut state, "result", &Option::<Value>::None)?;
        }
        ser::SerializeStruct::serialize_field(&mut state, "id", &self.id)?;
        ser::SerializeStruct::end(state)
    }
}

impl<'de> de::Deserialize<'de> for FailureResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor<'de> {
            marker: PhantomData<FailureResponse>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de> de::Visitor<'de> for Visitor<'de> {
            type Value = FailureResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct FailureResponse")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut jsonrpc = Option::<Version>::None;
                let mut result = Option::<Option<Value>>::None;
                let mut error = Option::<Error>::None;
                let mut id = Option::<Id>::None;

                while let Some(key) = de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Jsonrpc => {
                            if jsonrpc.is_some() {
                                return Err(de::Error::duplicate_field("jsonrpc"));
                            }
                            jsonrpc = Some(de::MapAccess::next_value::<Version>(&mut map)?)
                        }
                        Field::Result => {
                            if result.is_some() {
                                return Err(de::Error::duplicate_field("result"));
                            }
                            result = Some(de::MapAccess::next_value::<Option<Value>>(&mut map)?)
                        }
                        Field::Error => {
                            if error.is_some() {
                                return Err(de::Error::duplicate_field("error"));
                            }
                            error = Some(de::MapAccess::next_value::<Error>(&mut map)?)
                        }
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(de::MapAccess::next_value::<Id>(&mut map)?)
                        }
                    }
                }
                let (jsonrpc, error) = match (jsonrpc, result, error) {
                    (Some(version), None, Some(error)) => (Some(version), error),
                    (None, Some(value), Some(error)) if value.is_none() => (None, error),
                    (_, _, None) => return Err(de::Error::missing_field("error")),
                    _ => {
                        return Err(de::Error::custom(
                            "Incompatible with JSON-RPC specification v1 and v2",
                        ));
                    }
                };
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                Ok(FailureResponse { jsonrpc, error, id })
            }
        }

        de::Deserializer::deserialize_struct(
            deserializer,
            "FailureResponse",
            FIELDS,
            Visitor {
                marker: PhantomData::<FailureResponse>,
                lifetime: PhantomData,
            },
        )
    }
}

const FIELDS: &[&str] = &["jsonrpc", "result", "error", "id"];
enum Field {
    Jsonrpc,
    Result,
    Error,
    Id,
}
impl<'de> de::Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        de::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
    }
}

struct FieldVisitor;
impl<'de> de::Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("field identifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        match v {
            "jsonrpc" => Ok(Field::Jsonrpc),
            "result" => Ok(Field::Result),
            "error" => Ok(Field::Error),
            "id" => Ok(Field::Id),
            _ => Err(de::Error::unknown_field(v, &FIELDS)),
        }
    }
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
    fn invalid_response() {
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
}
