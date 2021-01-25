use std::fmt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_value, Map, Value};

use crate::response::Error;

/// Represents JSON-RPC request parameters.
///
/// If present, parameters for the rpc call MUST be provided as a Structured value.
/// Either by-position through an Array or by-name through an Object.
///
/// For JSON-RPC 1.0 specification, `params` **MUST** be an array of objects.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Params {
    /// Array of values
    Array(Vec<Value>),
    /// Map of values
    Map(Map<String, Value>),
}

impl Default for Params {
    fn default() -> Self {
        Params::Array(vec![])
    }
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Params {
    /// Parses incoming `Params` into expected types.
    pub fn parse<D>(self) -> Result<D, Error>
    where
        D: DeserializeOwned,
    {
        let value = self.into();
        from_value(value).map_err(Error::invalid_params)
    }

    /// Checks if there are no parameters for JSON-RPC 1.0, returns error if
    /// there are any parameters.
    pub fn expect_no_params_v1(self) -> Result<(), Error> {
        match self {
            Params::Array(ref v) if v.is_empty() => Ok(()),
            p => Err(Error::invalid_params_with_details(
                "No parameters were expected",
                p,
            )),
        }
    }

    /// Checks if the parameters is a array of objects.
    pub fn is_array(&self) -> bool {
        matches!(self, Params::Array(_))
    }

    /// Checks if the parameters is a map of objects.
    pub fn is_map(&self) -> bool {
        matches!(self, Params::Map(_))
    }
}

impl From<Params> for Value {
    fn from(params: Params) -> Value {
        match params {
            Params::Array(array) => Value::Array(array),
            Params::Map(object) => Value::Object(object),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialization() {
        let array = vec![Value::from(1), Value::Bool(true)];
        let params = Params::Array(array.clone());
        assert_eq!(serde_json::to_string(&params).unwrap(), r#"[1,true]"#);
        assert_eq!(
            serde_json::from_str::<Params>(r#"[1,true]"#).unwrap(),
            params
        );

        let object = {
            let mut map = Map::new();
            map.insert("key".into(), Value::String("value".into()));
            map
        };
        let params = Params::Map(object.clone());
        assert_eq!(
            serde_json::to_string(&params).unwrap(),
            r#"{"key":"value"}"#
        );
        assert_eq!(
            serde_json::from_str::<Params>(r#"{"key":"value"}"#).unwrap(),
            params
        );

        let params = Params::Array(vec![
            Value::Null,
            Value::Bool(true),
            Value::from(-1),
            Value::from(1),
            Value::from(1.2),
            Value::String("hello".to_string()),
            Value::Array(vec![]),
            Value::Array(array),
            Value::Object(object),
        ]);
        assert_eq!(
            serde_json::to_string(&params).unwrap(),
            r#"[null,true,-1,1,1.2,"hello",[],[1,true],{"key":"value"}]"#
        );
        assert_eq!(
            serde_json::from_str::<Params>(
                r#"[null,true,-1,1,1.2,"hello",[],[1,true],{"key":"value"}]"#
            )
            .unwrap(),
            params
        );
    }

    #[test]
    fn single_param_parsed_as_tuple() {
        let params: (u64,) = Params::Array(vec![Value::from(1)]).parse().unwrap();
        assert_eq!(params, (1,));
    }

    #[test]
    fn invalid_params() {
        let params = serde_json::from_str::<Params>("[true]").unwrap();
        assert_eq!(
            params.clone().expect_no_params_v1().unwrap_err(),
            Error::invalid_params_with_details("No parameters were expected", params)
        );

        let params = serde_json::from_str::<Params>("[1,true]").unwrap();
        assert_eq!(
            params.parse::<(u8, bool, String)>().unwrap_err(),
            Error::invalid_params("invalid length 2, expected a tuple of size 3")
        );
    }
}
