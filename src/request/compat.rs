use std::fmt;
use std::marker::PhantomData;

use serde::{de, ser};

use crate::id::Id;
use crate::request::{MethodCall, Notification, Params};
use crate::version::Version;

const FIELDS: &[&str] = &["jsonrpc", "method", "params", "id"];

enum Field {
    Jsonrpc,
    Method,
    Params,
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
            "method" => Ok(Field::Method),
            "params" => Ok(Field::Params),
            "id" => Ok(Field::Id),
            _ => Err(de::Error::unknown_field(v, &FIELDS)),
        }
    }
}

impl ser::Serialize for MethodCall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match &self.jsonrpc {
            // JSON-RPC v2
            Some(Version::V2_0) => {
                if self.params.is_some() {
                    let mut state = ser::Serializer::serialize_struct(serializer, "MethodCall", 4)?;
                    ser::SerializeStruct::serialize_field(&mut state, "jsonrpc", &self.jsonrpc)?;
                    ser::SerializeStruct::serialize_field(&mut state, "method", &self.method)?;
                    ser::SerializeStruct::serialize_field(&mut state, "params", &self.params)?;
                    ser::SerializeStruct::serialize_field(&mut state, "id", &self.id)?;
                    ser::SerializeStruct::end(state)
                } else {
                    let mut state = ser::Serializer::serialize_struct(serializer, "MethodCall", 3)?;
                    ser::SerializeStruct::serialize_field(&mut state, "jsonrpc", &self.jsonrpc)?;
                    ser::SerializeStruct::serialize_field(&mut state, "method", &self.method)?;
                    ser::SerializeStruct::skip_field(&mut state, "params")?;
                    ser::SerializeStruct::serialize_field(&mut state, "id", &self.id)?;
                    ser::SerializeStruct::end(state)
                }
            }
            // JSON-RPC v1
            None => {
                let mut state = ser::Serializer::serialize_struct(serializer, "MethodCall", 3)?;
                ser::SerializeStruct::skip_field(&mut state, "jsonrpc")?;
                ser::SerializeStruct::serialize_field(&mut state, "method", &self.method)?;
                match &self.params {
                    Some(Params::Array(_)) => {
                        ser::SerializeStruct::serialize_field(&mut state, "params", &self.params)?
                    }
                    Some(Params::Map(_)) => {
                        return Err(ser::Error::custom(
                            "JSON-RPC v1 params must be an array of objects",
                        ));
                    }
                    None => return Err(ser::Error::custom("missing field `params`")),
                }
                ser::SerializeStruct::serialize_field(&mut state, "id", &self.id)?;
                ser::SerializeStruct::end(state)
            }
        }
    }
}

impl<'de> de::Deserialize<'de> for MethodCall {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor<'de> {
            marker: PhantomData<MethodCall>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de> de::Visitor<'de> for Visitor<'de> {
            type Value = MethodCall;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct MethodCall")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut jsonrpc = Option::<Version>::None;
                let mut method = Option::<String>::None;
                let mut params = Option::<Params>::None;
                let mut id = Option::<Id>::None;

                while let Some(key) = de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Jsonrpc => {
                            if jsonrpc.is_some() {
                                return Err(de::Error::duplicate_field("jsonrpc"));
                            }
                            jsonrpc = Some(de::MapAccess::next_value::<Version>(&mut map)?)
                        }
                        Field::Method => {
                            if method.is_some() {
                                return Err(de::Error::duplicate_field("method"));
                            }
                            method = Some(de::MapAccess::next_value::<String>(&mut map)?)
                        }
                        Field::Params => {
                            if params.is_some() {
                                return Err(de::Error::duplicate_field("params"));
                            }
                            params = Some(de::MapAccess::next_value::<Params>(&mut map)?)
                        }
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(de::MapAccess::next_value::<Id>(&mut map)?)
                        }
                    }
                }

                let params = match (jsonrpc, params) {
                    // JSON-RPC v2
                    (Some(Version::V2_0), params) => params,
                    // JSON-RPC v2
                    (None, Some(params)) => {
                        if let Params::Array(_) = params {
                            Some(params)
                        } else {
                            return Err(de::Error::custom(
                                "JSON-RPC v1 params must be an array of objects",
                            ));
                        }
                    }
                    // Others
                    _ => {
                        return Err(de::Error::custom(
                            "Incompatible with JSON-RPC v1 and v2 specification",
                        ));
                    }
                };
                let method = method.ok_or_else(|| de::Error::missing_field("method"))?;
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                Ok(MethodCall {
                    jsonrpc,
                    method,
                    params,
                    id,
                })
            }
        }

        de::Deserializer::deserialize_struct(
            deserializer,
            "MethodCall",
            FIELDS,
            Visitor {
                marker: PhantomData::<MethodCall>,
                lifetime: PhantomData,
            },
        )
    }
}

impl ser::Serialize for Notification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match (&self.jsonrpc, &self.params) {
            // JSON-RPC v2
            (Some(Version::V2_0), Some(params)) => {
                let mut state = ser::Serializer::serialize_struct(serializer, "Notification", 3)?;
                ser::SerializeStruct::serialize_field(&mut state, "jsonrpc", &self.jsonrpc)?;
                ser::SerializeStruct::serialize_field(&mut state, "method", &self.method)?;
                ser::SerializeStruct::serialize_field(&mut state, "params", params)?;
                ser::SerializeStruct::end(state)
            }
            (Some(Version::V2_0), None) => {
                let mut state = ser::Serializer::serialize_struct(serializer, "Notification", 2)?;
                ser::SerializeStruct::serialize_field(&mut state, "jsonrpc", &self.jsonrpc)?;
                ser::SerializeStruct::serialize_field(&mut state, "method", &self.method)?;
                ser::SerializeStruct::skip_field(&mut state, "params")?;
                ser::SerializeStruct::end(state)
            }
            // JSON-RPC v1
            (None, Some(params)) => {
                let mut state = ser::Serializer::serialize_struct(serializer, "Notification", 3)?;
                ser::SerializeStruct::skip_field(&mut state, "jsonrpc")?;
                ser::SerializeStruct::serialize_field(&mut state, "method", &self.method)?;
                match params {
                    Params::Array(_) => {
                        ser::SerializeStruct::serialize_field(&mut state, "params", params)?
                    }
                    Params::Map(_) => {
                        return Err(ser::Error::custom(
                            "JSON-RPC v1 params must be an array of objects",
                        ));
                    }
                }
                ser::SerializeStruct::serialize_field(&mut state, "id", &Option::<Id>::None)?;
                ser::SerializeStruct::end(state)
            }
            (None, None) => Err(ser::Error::custom(
                "Incompatible with JSON-RPC v1 and v2 specification",
            )),
        }
    }
}

impl<'de> de::Deserialize<'de> for Notification {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor<'de> {
            marker: PhantomData<Notification>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de> de::Visitor<'de> for Visitor<'de> {
            type Value = Notification;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Notification")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut jsonrpc = Option::<Version>::None;
                let mut method = Option::<String>::None;
                let mut params = Option::<Params>::None;
                let mut id = Option::<Option<Id>>::None;

                while let Some(key) = de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Jsonrpc => {
                            if jsonrpc.is_some() {
                                return Err(de::Error::duplicate_field("jsonrpc"));
                            }
                            jsonrpc = Some(de::MapAccess::next_value::<Version>(&mut map)?)
                        }
                        Field::Method => {
                            if method.is_some() {
                                return Err(de::Error::duplicate_field("method"));
                            }
                            method = Some(de::MapAccess::next_value::<String>(&mut map)?)
                        }
                        Field::Params => {
                            if params.is_some() {
                                return Err(de::Error::duplicate_field("params"));
                            }
                            params = Some(de::MapAccess::next_value::<Params>(&mut map)?)
                        }
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(de::MapAccess::next_value::<Option<Id>>(&mut map)?)
                        }
                    }
                }

                let method = method.ok_or_else(|| de::Error::missing_field("method"))?;
                let params = match (jsonrpc, params, id) {
                    // JSON-RPC v2
                    (Some(Version::V2_0), params, None) => params,
                    (Some(Version::V2_0), _, Some(_)) => {
                        return Err(de::Error::custom(
                            "JSON-RPC v2 notification must not contain id",
                        ));
                    }
                    // JSON-RPC v1
                    (None, Some(params), Some(None)) => {
                        if let Params::Array(_) = params {
                            Some(params)
                        } else {
                            return Err(de::Error::custom(
                                "JSON-RPC v1 params must be an array of objects, id must be null",
                            ));
                        }
                    }
                    // Others
                    _ => {
                        return Err(de::Error::custom(
                            "Incompatible with JSON-RPC v1 and v2 specification",
                        ));
                    }
                };
                Ok(Notification {
                    jsonrpc,
                    method,
                    params,
                })
            }
        }

        de::Deserializer::deserialize_struct(
            deserializer,
            "Notification",
            FIELDS,
            Visitor {
                marker: PhantomData::<Notification>,
                lifetime: PhantomData,
            },
        )
    }
}
