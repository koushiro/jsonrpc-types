use std::{fmt, marker::PhantomData};

use serde::{de, Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::{Error, ErrorCode},
    id::Id,
};

/// Represents success / failure output of JSON-RPC 1.0 response.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Output {
    /// Successful execution result.
    pub result: Option<Value>,
    /// Failed execution error.
    pub error: Option<Error>,
    /// Correlation id.
    ///
    /// It **MUST** be the same as the value of the id member in the Request Object.
    pub id: Id,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).expect("`Output` is serializable");
        write!(f, "{}", json)
    }
}

impl<'de> de::Deserialize<'de> for Output {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        use self::response_field::{Field, FIELDS};

        struct Visitor<'de> {
            marker: PhantomData<Output>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de> de::Visitor<'de> for Visitor<'de> {
            type Value = Output;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Output")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut result = Option::<Option<Value>>::None;
                let mut error = Option::<Option<Error>>::None;
                let mut id = Option::<Id>::None;

                while let Some(key) = de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
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

                let (result, error) =
                    match (result, error) {
                        (Some(Some(value)), Some(None)) => (Some(value), None),
                        (Some(None), Some(Some(error))) => (None, Some(error)),
                        _ => return Err(de::Error::custom(
                            "JSON-RPC 1.0 response MUST contain both `result` and `error` field",
                        )),
                    };
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                Ok(Output { result, error, id })
            }
        }

        de::Deserializer::deserialize_struct(
            deserializer,
            "Output",
            FIELDS,
            Visitor {
                marker: PhantomData::<Output>,
                lifetime: PhantomData,
            },
        )
    }
}

impl Output {
    /// Creates a new response output with given `result` and `id`.
    pub fn new(result: Result<Value, Error>, id: Id) -> Self {
        match result {
            Ok(result) => Output::success(result, id),
            Err(error) => Output::failure(error, id),
        }
    }

    /// Creates a JSON-RPC 1.0 success response output.
    pub fn success(result: Value, id: Id) -> Self {
        Self {
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Creates a JSON-RPC 1.0 failure response output.
    pub fn failure(error: Error, id: Id) -> Self {
        Self {
            result: None,
            error: Some(error),
            id,
        }
    }

    /// Creates a new failure response output indicating malformed request.
    pub fn invalid_request(id: Id) -> Self {
        Output::failure(Error::new(ErrorCode::InvalidRequest), id)
    }
}

impl From<Output> for Result<Value, Error> {
    // Convert into a result.
    // Will be `Ok` if it is a `SuccessResponse` and `Err` if `FailureResponse`.
    fn from(output: Output) -> Result<Value, Error> {
        match (output.result, output.error) {
            (Some(result), None) => Ok(result),
            (None, Some(error)) => Err(error),
            _ => unreachable!("Invalid JSON-RPC 1.0 Response"),
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

mod response_field {
    use super::*;

    pub const FIELDS: &[&str] = &["result", "error", "id"];
    pub enum Field {
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
                "result" => Ok(Field::Result),
                "error" => Ok(Field::Error),
                "id" => Ok(Field::Id),
                _ => Err(de::Error::unknown_field(v, &FIELDS)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response_output_cases() -> Vec<(Output, &'static str)> {
        vec![
            (
                // JSON-RPC 1.0 success response output
                Output {
                    result: Some(Value::Bool(true)),
                    error: None,
                    id: Id::Num(1),
                },
                r#"{"result":true,"error":null,"id":1}"#,
            ),
            (
                // JSON-RPC 1.0 failure response output
                Output {
                    result: None,
                    error: Some(Error::parse_error()),
                    id: Id::Num(1),
                },
                r#"{"result":null,"error":{"code":-32700,"message":"Parse error"},"id":1}"#,
            ),
        ]
    }

    #[test]
    fn response_output_serialization() {
        for (success_response, expect) in response_output_cases() {
            let ser = serde_json::to_string(&success_response).unwrap();
            assert_eq!(ser, expect);
            let de = serde_json::from_str::<Output>(expect).unwrap();
            assert_eq!(de, success_response);
        }
    }

    #[test]
    fn response_serialization() {
        for (output, expect) in response_output_cases() {
            let response = Response::Single(output);
            assert_eq!(serde_json::to_string(&response).unwrap(), expect);
            assert_eq!(serde_json::from_str::<Response>(expect).unwrap(), response);
        }

        let batch_response = Response::Batch(vec![
            Output {
                result: Some(Value::Bool(true)),
                error: None,
                id: Id::Num(1),
            },
            Output {
                result: Some(Value::Bool(false)),
                error: None,
                id: Id::Num(2),
            },
        ]);
        let batch_expect =
            r#"[{"result":true,"error":null,"id":1},{"result":false,"error":null,"id":2}]"#;
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
            // JSON-RPC 1.0 invalid response
            r#"{"result":true,"error":null,"id":1,unknown:[]}"#,
            r#"{"result":true,"error":{"code": -32700,"message": "Parse error"},"id":1}"#,
            r#"{"result":true,"error":{"code": -32700,"message": "Parse error"}}"#,
            r#"{"result":true,"id":1}"#,
            r#"{"error":{"code": -32700,"message": "Parse error"},"id":1}"#,
            r#"{"unknown":[]}"#,
        ];

        for case in cases {
            let response = serde_json::from_str::<Response>(case);
            assert!(response.is_err());
        }
    }

    #[test]
    fn valid_response() {
        let cases = vec![
            // JSON-RPC 1.0 valid response
            r#"{"result":true,"error":null,"id":1}"#,
            r#"{"result":null,"error":{"code": -32700,"message": "Parse error"},"id":1}"#,
        ];

        for case in cases {
            let response = serde_json::from_str::<Response>(case);
            assert!(response.is_ok());
        }
    }
}
