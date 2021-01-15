mod params;

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::version::Version;

pub use self::params::Params;

/// Represents JSON-RPC request which is a method call.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MethodCall {
    /// A String specifying the version of the JSON-RPC protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonrpc: Option<Version>,
    /// A String containing the name of the method to be invoked.
    ///
    /// Method names that begin with the word rpc followed by a period character (U+002E or ASCII 46)
    /// are reserved for rpc-internal methods and extensions and MUST NOT be used for anything else.
    pub method: String,
    /// A Structured value that holds the parameter values to be used
    /// during the invocation of the method. This member MAY be omitted.
    ///
    /// For compatibility with JSON-RPC v1 specification, params **MUST** be an array of objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
    /// An identifier established by the Client.
    /// If it is not included it is assumed to be a notification.
    pub id: Id,
}

/// Represents JSON-RPC request which is a notification.
///
/// A Request object that is a Notification signifies the Client's lack of interest in the
/// corresponding Response object, and as such no Response object needs to be returned to the client.
/// As such, the Client would not be aware of any errors (like e.g. "Invalid params","Internal error").
///
/// The Server MUST NOT reply to a Notification, including those that are within a batch request.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Notification {
    /// A String specifying the version of the JSON-RPC protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonrpc: Option<Version>,
    /// A String containing the name of the method to be invoked.
    ///
    /// Method names that begin with the word rpc followed by a period character (U+002E or ASCII 46)
    /// are reserved for rpc-internal methods and extensions and MUST NOT be used for anything else.
    pub method: String,
    /// A Structured value that holds the parameter values to be used
    /// during the invocation of the method. This member MAY be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
    // For compatibility with JSON-RPC v1 specification, id **MUST** be Null.
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

impl From<MethodCall> for Call {
    fn from(call: MethodCall) -> Self {
        Call::MethodCall(call)
    }
}

impl From<Notification> for Call {
    fn from(notify: Notification) -> Self {
        Call::Notification(notify)
    }
}

/// JSON-RPC Request object.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Request {
    /// Single request (call)
    Single(Call),
    /// Batch of requests (calls)
    Batch(Vec<Call>),
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    fn method_call_cases() -> Vec<(MethodCall, &'static str)> {
        vec![
            (
                // JSON-RPC v1 request method call
                MethodCall {
                    jsonrpc: None,
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![Value::from(1), Value::Bool(true)])),
                    id: Id::Num(1),
                },
                r#"{"method":"foo","params":[1,true],"id":1}"#,
            ),
            (
                // JSON-RPC v1 request method call without parameters
                MethodCall {
                    jsonrpc: None,
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![])),
                    id: Id::Num(1),
                },
                r#"{"method":"foo","params":[],"id":1}"#,
            ),
            (
                // JSON-RPC v2 request method call
                MethodCall {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![Value::from(1), Value::Bool(true)])),
                    id: Id::Num(1),
                },
                r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":1}"#,
            ),
            (
                // JSON-RPC v2 request method call with an empty array parameters
                MethodCall {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![])),
                    id: Id::Num(1),
                },
                r#"{"jsonrpc":"2.0","method":"foo","params":[],"id":1}"#,
            ),
            (
                // JSON-RPC v2 request method call without parameters
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
                // JSON-RPC v1 request notification
                Notification {
                    jsonrpc: None,
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![Value::from(1), Value::Bool(true)])),
                },
                // r#"{"method":"foo","params":[1,true], "id":null}"#,
                r#"{"method":"foo","params":[1,true]}"#,
            ),
            (
                // JSON-RPC v1 request notification without parameters
                Notification {
                    jsonrpc: None,
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![])),
                },
                // r#"{"method":"foo","params":[],"id":null}"#,
                r#"{"method":"foo","params":[]}"#,
            ),
            (
                // JSON-RPC v2 request notification
                Notification {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![Value::from(1), Value::Bool(true)])),
                },
                r#"{"jsonrpc":"2.0","method":"foo","params":[1,true]}"#,
            ),
            (
                // JSON-RPC v2 request method call with an empty array parameters
                Notification {
                    jsonrpc: Some(Version::V2_0),
                    method: "foo".to_string(),
                    params: Some(Params::Array(vec![])),
                },
                r#"{"jsonrpc":"2.0","method":"foo","params":[]}"#,
            ),
            (
                // JSON-RPC v2 request notification without parameters
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
            println!("{}", batch_expect);
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
            // JSON-RPC v1 invalid request
            r#"{"method":"foo","params":[1,true],"id":null,"unknown":[]}"#,
            r#"{"method":"foo","params":[1,true],"id":1,"unknown":[]}"#,
            r#"{"method":"foo","params":[1,true],"unknown":[]}"#,
            r#"{"method":"foo","unknown":[]}"#,
            r#"{"unknown":[]}"#,
            // JSON-RPC v2 invalid request
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":null,"unknown":[]}"#,
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":1,"unknown":[]}"#,
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"unknown":[]}"#,
            r#"{"jsonrpc":"2.0","method":"foo","unknown":[]}"#,
            r#"{"jsonrpc":"2.0","unknown":[]}"#,
        ];

        for case in cases {
            let request = serde_json::from_str::<Request>(case);
            assert!(request.is_err());
        }
    }
}
