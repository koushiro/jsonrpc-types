use std::fmt;
use std::marker::PhantomData;

use serde::{de, ser};
use serde_json::Value;

use crate::id::Id;
use crate::response::{Error, FailureResponse, SuccessResponse};
use crate::version::Version;

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

impl ser::Serialize for SuccessResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self.jsonrpc {
            // JSON-RPC v2
            Some(Version::V2_0) => {
                let mut state =
                    ser::Serializer::serialize_struct(serializer, "SuccessResponse", 3)?;
                ser::SerializeStruct::serialize_field(&mut state, "jsonrpc", &self.jsonrpc)?;
                ser::SerializeStruct::serialize_field(&mut state, "result", &self.result)?;
                ser::SerializeStruct::serialize_field(&mut state, "id", &self.id)?;
                ser::SerializeStruct::end(state)
            }
            // JSON-RPC v1
            None => {
                let mut state =
                    ser::Serializer::serialize_struct(serializer, "SuccessResponse", 3)?;
                ser::SerializeStruct::serialize_field(&mut state, "result", &self.result)?;
                ser::SerializeStruct::serialize_field(&mut state, "error", &Option::<Error>::None)?;
                ser::SerializeStruct::serialize_field(&mut state, "id", &self.id)?;
                ser::SerializeStruct::end(state)
            }
        }
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

                let result = match (jsonrpc, result, error) {
                    // JSON-RPC v2
                    (Some(Version::V2_0), Some(value), None) => value,
                    // JSON-RPC v1
                    (None, Some(value), Some(None)) => value,
                    // Others
                    _ => {
                        return Err(de::Error::custom(
                            "Incompatible with JSON-RPC v1 and v2 specification",
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

impl ser::Serialize for FailureResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self.jsonrpc {
            // JSON-RPC v2
            Some(Version::V2_0) => {
                let mut state =
                    ser::Serializer::serialize_struct(serializer, "FailureResponse", 3)?;
                ser::SerializeStruct::serialize_field(&mut state, "jsonrpc", &self.jsonrpc)?;
                ser::SerializeStruct::serialize_field(&mut state, "error", &self.error)?;
                ser::SerializeStruct::serialize_field(&mut state, "id", &self.id)?;
                ser::SerializeStruct::end(state)
            }
            // JSON-RPC v1
            None => {
                let mut state =
                    ser::Serializer::serialize_struct(serializer, "FailureResponse", 3)?;
                ser::SerializeStruct::serialize_field(&mut state, "error", &self.error)?;
                ser::SerializeStruct::serialize_field(
                    &mut state,
                    "result",
                    &Option::<Value>::None,
                )?;
                ser::SerializeStruct::serialize_field(&mut state, "id", &self.id)?;
                ser::SerializeStruct::end(state)
            }
        }
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

                let error = match (jsonrpc, result, error) {
                    // JSON-RPC v2
                    (Some(Version::V2_0), None, Some(error)) => error,
                    // JSON-RPC v1
                    (None, Some(None), Some(error)) => error,
                    // Others
                    _ => {
                        return Err(de::Error::custom(
                            "Incompatible with JSON-RPC v1 and v2 specification",
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
