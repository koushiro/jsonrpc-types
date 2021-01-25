// For compatibility with JSON-RPC 1.0 specification.
mod compat;
mod params;

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::version::Version;

pub use self::params::Params;

/// Represents JSON-RPC request which is a method call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodCall {
    /// A String specifying the version of the JSON-RPC protocol.
    pub jsonrpc: Option<Version>,
    /// A String containing the name of the method to be invoked.
    ///
    /// Method names that begin with the word rpc followed by a period character (U+002E or ASCII 46)
    /// are reserved for rpc-internal methods and extensions and MUST NOT be used for anything else.
    pub method: String,
    /// A Structured value that holds the parameter values to be used
    /// during the invocation of the method. This member MAY be omitted.
    ///
    /// For JSON-RPC 1.0 specification, params **MUST** be an array of objects.
    pub params: Option<Params>,
    /// An identifier established by the Client.
    /// If it is not included it is assumed to be a notification.
    pub id: Id,
}

impl fmt::Display for MethodCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).expect("`MethodCall` is serializable");
        write!(f, "{}", json)
    }
}

impl MethodCall {
    /// Creates a JSON-RPC 1.0 request which is a method call.
    pub fn new_v1<M: Into<String>>(method: M, params: Params, id: Id) -> Self {
        assert!(
            params.is_array(),
            "`params` must be an array of objects for JSON-RPC 1.0"
        );
        Self {
            jsonrpc: None,
            method: method.into(),
            params: Some(params),
            id,
        }
    }

    /// Creates a JSON-RPC 2.0 request which is a method call.
    pub fn new_v2<M: Into<String>>(method: M, params: Option<Params>, id: Id) -> Self {
        Self {
            jsonrpc: Some(Version::V2_0),
            method: method.into(),
            params,
            id,
        }
    }
}

/// Represents JSON-RPC request which is a notification.
///
/// A Request object that is a Notification signifies the Client's lack of interest in the
/// corresponding Response object, and as such no Response object needs to be returned to the client.
/// As such, the Client would not be aware of any errors (like e.g. "Invalid params","Internal error").
///
/// The Server MUST NOT reply to a Notification, including those that are within a batch request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Notification {
    /// A String specifying the version of the JSON-RPC protocol.
    pub jsonrpc: Option<Version>,
    /// A String containing the name of the method to be invoked.
    ///
    /// Method names that begin with the word rpc followed by a period character (U+002E or ASCII 46)
    /// are reserved for rpc-internal methods and extensions and MUST NOT be used for anything else.
    pub method: String,
    /// A Structured value that holds the parameter values to be used
    /// during the invocation of the method. This member MAY be omitted.
    pub params: Option<Params>,
    // For JSON-RPC 1.0 specification, id **MUST** be Null.
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).expect("`Notification` is serializable");
        write!(f, "{}", json)
    }
}

impl Notification {
    /// Creates a JSON-RPC 1.0 request which is a notification.
    pub fn new_v1<M: Into<String>>(method: M, params: Params) -> Self {
        assert!(
            params.is_array(),
            "`params` must be an array of objects for JSON-RPC 1.0"
        );
        Self {
            jsonrpc: None,
            method: method.into(),
            params: Some(params),
        }
    }

    /// Creates a JSON-RPC 2.0 request which is a notification.
    pub fn new_v2<M: Into<String>>(method: M, params: Option<Params>) -> Self {
        Self {
            jsonrpc: Some(Version::V2_0),
            method: method.into(),
            params,
        }
    }
}

/// Represents single JSON-RPC call.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Call {
    /// Call method
    MethodCall(MethodCall),
    /// Fire notification
    Notification(Notification),
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).expect("`Call` is serializable");
        write!(f, "{}", json)
    }
}

impl Call {
    /// Returns the method of the request call.
    pub fn method(&self) -> &str {
        match self {
            Self::MethodCall(call) => &call.method,
            Self::Notification(notification) => &notification.method,
        }
    }

    /// Returns the params of the request call.
    pub fn params(&self) -> &Option<Params> {
        match self {
            Self::MethodCall(call) => &call.params,
            Self::Notification(notification) => &notification.params,
        }
    }

    /// Returns the id of the request call.
    pub fn id(&self) -> Option<Id> {
        match self {
            Self::MethodCall(call) => Some(call.id.clone()),
            Self::Notification(_notification) => None,
        }
    }
}

impl From<MethodCall> for Call {
    fn from(call: MethodCall) -> Self {
        Self::MethodCall(call)
    }
}

impl From<Notification> for Call {
    fn from(notify: Notification) -> Self {
        Self::Notification(notify)
    }
}

/// JSON-RPC Request object.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Request {
    /// Single call
    Single(Call),
    /// Batch of calls
    Batch(Vec<Call>),
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).expect("`Request` is serializable");
        write!(f, "{}", json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn method_call_cases() -> Vec<(MethodCall, &'static str)> {
        vec![
            (
                // JSON-RPC 1.0 request method call
                MethodCall {
                    jsonrpc: None,
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![Value::from(1), Value::Bool(true)])),
                    id: Id::Num(1),
                },
                r#"{"method":"foo","params":[1,true],"id":1}"#,
            ),
            (
                // JSON-RPC 1.0 request method call without parameters
                MethodCall {
                    jsonrpc: None,
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![])),
                    id: Id::Num(1),
                },
                r#"{"method":"foo","params":[],"id":1}"#,
            ),
            (
                // JSON-RPC 2.0 request method call
                MethodCall {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![Value::from(1), Value::Bool(true)])),
                    id: Id::Num(1),
                },
                r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":1}"#,
            ),
            (
                // JSON-RPC 2.0 request method call with an empty array parameters
                MethodCall {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![])),
                    id: Id::Num(1),
                },
                r#"{"jsonrpc":"2.0","method":"foo","params":[],"id":1}"#,
            ),
            (
                // JSON-RPC 2.0 request method call without parameters
                MethodCall {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: None,
                    id: Id::Num(1),
                },
                r#"{"jsonrpc":"2.0","method":"foo","id":1}"#,
            ),
        ]
    }

    fn notification_cases() -> Vec<(Notification, &'static str)> {
        vec![
            (
                // JSON-RPC 1.0 request notification
                Notification {
                    jsonrpc: None,
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![Value::from(1), Value::Bool(true)])),
                },
                r#"{"method":"foo","params":[1,true],"id":null}"#,
            ),
            (
                // JSON-RPC 1.0 request notification without parameters
                Notification {
                    jsonrpc: None,
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![])),
                },
                r#"{"method":"foo","params":[],"id":null}"#,
            ),
            (
                // JSON-RPC 2.0 request notification
                Notification {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![Value::from(1), Value::Bool(true)])),
                },
                r#"{"jsonrpc":"2.0","method":"foo","params":[1,true]}"#,
            ),
            (
                // JSON-RPC 2.0 request method call with an empty array parameters
                Notification {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![])),
                },
                r#"{"jsonrpc":"2.0","method":"foo","params":[]}"#,
            ),
            (
                // JSON-RPC 2.0 request notification without parameters
                Notification {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: None,
                },
                r#"{"jsonrpc":"2.0","method":"foo"}"#,
            ),
        ]
    }

    #[test]
    fn method_call_serialization() {
        for (method_call, expect) in method_call_cases() {
            let ser = serde_json::to_string(&method_call).unwrap();
            assert_eq!(ser, expect);
            let de = serde_json::from_str::<MethodCall>(expect).unwrap();
            assert_eq!(de, method_call);
        }
    }

    #[test]
    fn notification_serialization() {
        for (notification, expect) in notification_cases() {
            let ser = serde_json::to_string(&notification).unwrap();
            assert_eq!(ser, expect);
            let de = serde_json::from_str::<Notification>(expect).unwrap();
            assert_eq!(de, notification);
        }
    }

    #[test]
    fn call_serialization() {
        for (method_call, expect) in method_call_cases() {
            let call = Call::MethodCall(method_call);
            assert_eq!(serde_json::to_string(&call).unwrap(), expect);
            assert_eq!(serde_json::from_str::<Call>(expect).unwrap(), call);
        }

        for (notification, expect) in notification_cases() {
            let call = Call::Notification(notification);
            assert_eq!(serde_json::to_string(&call).unwrap(), expect);
            assert_eq!(serde_json::from_str::<Call>(expect).unwrap(), call);
        }
    }

    #[test]
    fn request_serialization() {
        for (method_call, expect) in method_call_cases() {
            let call_request = Request::Single(Call::MethodCall(method_call));
            assert_eq!(serde_json::to_string(&call_request).unwrap(), expect);
            assert_eq!(
                serde_json::from_str::<Request>(expect).unwrap(),
                call_request
            );
        }

        for (notification, expect) in notification_cases() {
            let notification_request = Request::Single(Call::Notification(notification));
            assert_eq!(
                serde_json::to_string(&notification_request).unwrap(),
                expect
            );
            assert_eq!(
                serde_json::from_str::<Request>(expect).unwrap(),
                notification_request
            );
        }

        for ((call, call_expect), (notification, notification_expect)) in
            method_call_cases().into_iter().zip(notification_cases())
        {
            let batch_request = Request::Batch(vec![
                Call::MethodCall(call),
                Call::Notification(notification),
            ]);
            let batch_expect = format!("[{},{}]", call_expect, notification_expect);
            assert_eq!(serde_json::to_string(&batch_request).unwrap(), batch_expect);
            assert_eq!(
                serde_json::from_str::<Request>(&batch_expect).unwrap(),
                batch_request
            );
        }
    }

    #[test]
    fn invalid_request() {
        let cases = vec![
            // JSON-RPC 1.0 invalid request
            r#"{"method":"foo","params":[1,true],"id":1,"unknown":[]}"#,
            r#"{"method":"foo","params":[1,true],"id":1.2}"#,
            r#"{"method":"foo","params":[1,true],"id":null,"unknown":[]}"#,
            r#"{"method":"foo","params":[1,true],"unknown":[]}"#,
            r#"{"method":"foo","params":[1,true]}"#,
            r#"{"method":"foo","unknown":[]}"#,
            r#"{"method":1,"unknown":[]}"#,
            r#"{"unknown":[]}"#,
            // JSON-RPC 2.0 invalid request
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":1,"unknown":[]}"#,
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":1.2}"#,
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":null,"unknown":[]}"#,
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":null}"#,
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"unknown":[]}"#,
            r#"{"jsonrpc":"2.0","method":"foo","unknown":[]}"#,
            r#"{"jsonrpc":"2.0","unknown":[]}"#,
        ];

        for case in cases {
            let request = serde_json::from_str::<Request>(case);
            assert!(request.is_err());
        }
    }

    #[test]
    fn valid_request() {
        let cases = vec![
            // JSON-RPC 1.0 valid request
            r#"{"method":"foo","params":[1,true],"id":1}"#,
            r#"{"method":"foo","params":[],"id":1}"#,
            r#"{"method":"foo","params":[1,true],"id":null}"#,
            r#"{"method":"foo","params":[],"id":null}"#,
            // JSON-RPC 2.0 valid request
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":1}"#,
            r#"{"jsonrpc":"2.0","method":"foo","params":[],"id":1}"#,
            r#"{"jsonrpc":"2.0","method":"foo","id":1}"#,
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true]}"#,
            r#"{"jsonrpc":"2.0","method":"foo","params":[]}"#,
            r#"{"jsonrpc":"2.0","method":"foo"}"#,
        ];

        for case in cases {
            let request = serde_json::from_str::<Request>(case);
            assert!(request.is_ok());
        }
    }
}
