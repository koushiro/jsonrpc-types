use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::error::Error;

/// Request parameters
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Params {
    /// No parameters
    None,
    /// Array of values
    Array(Vec<JsonValue>),
    /// Map of values
    Map(JsonMap<String, JsonValue>),
}

impl Params {
    /// Parse incoming `Params` into expected types.
    pub fn parse<D>(self) -> Result<D, Error>
    where
        D: DeserializeOwned,
    {
        let value = match self {
            Params::Array(vec) => JsonValue::Array(vec),
            Params::Map(map) => JsonValue::Object(map),
            Params::None => JsonValue::Null,
        };

        serde_json::from_value(value)
            .map_err(|err| Error::invalid_params(format!("Invalid params: {}.", err)))
    }

    /// Check for no params, returns Err if any params
    pub fn expect_no_params(self) -> Result<(), Error> {
        match self {
            Params::None => Ok(()),
            Params::Array(ref v) if v.is_empty() => Ok(()),
            p => Err(Error::invalid_params_with_details(
                "No parameters were expected",
                p,
            )),
        }
    }
}

impl From<Params> for JsonValue {
    fn from(params: Params) -> JsonValue {
        match params {
            Params::Array(vec) => JsonValue::Array(vec),
            Params::Map(map) => JsonValue::Object(map),
            Params::None => JsonValue::Null,
        }
    }
}
