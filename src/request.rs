use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::params::Params;
use crate::version::Version;

/// Represents JSON-RPC request which is a method call.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
    #[serde(default)]
    pub params: Params,
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
    pub jsonrpc: Option<Version>,
    /// A String containing the name of the method to be invoked.
    ///
    /// Method names that begin with the word rpc followed by a period character (U+002E or ASCII 46)
    /// are reserved for rpc-internal methods and extensions and MUST NOT be used for anything else.
    pub method: String,
    /// A Structured value that holds the parameter values to be used
    /// during the invocation of the method. This member MAY be omitted.
    #[serde(default)]
    pub params: Params,
}

/// Represents single JSON-RPC call.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Call {
    /// Call method
    MethodCall(MethodCall),
    /// Fire notification
    Notification(Notification),
    /// Invalid call
    Invalid {
        /// Call id (if known)
        #[serde(default)]
        id: Id,
    },
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
    use super::*;
    use serde_json::Value;

    fn method_call() -> (MethodCall, &'static str) {
        (
            MethodCall {
                jsonrpc: Some(Version::V2),
                method: "foo".to_string(),
                params: Params::Array(vec![Value::from(1), Value::Bool(true)]),
                id: Id::Num(1),
            },
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true],"id":1}"#,
        )
    }

    fn notification() -> (Notification, &'static str) {
        (
            Notification {
                jsonrpc: Some(Version::V2),
                method: "foo".to_string(),
                params: Params::Array(vec![Value::from(1), Value::Bool(true)]),
            },
            r#"{"jsonrpc":"2.0","method":"foo","params":[1,true]}"#,
        )
    }

    #[test]
    fn method_call_serialization() {
        let (method_call, expect) = method_call();
        let ser = serde_json::to_string(&method_call).unwrap();
        assert_eq!(ser, expect);
        let de = serde_json::from_str::<MethodCall>(expect).unwrap();
        assert_eq!(de, method_call);
    }

    #[test]
    fn notification_serialization() {
        let (notification, expect) = notification();
        let ser = serde_json::to_string(&notification).unwrap();
        assert_eq!(ser, expect);
        let de = serde_json::from_str::<Notification>(expect).unwrap();
        assert_eq!(de, notification);
    }

    #[test]
    fn call_serialization() {
        let (method_call, expect) = method_call();
        let call = Call::MethodCall(method_call);
        assert_eq!(serde_json::to_string(&call).unwrap(), expect);
        assert_eq!(serde_json::from_str::<Call>(expect).unwrap(), call);

        let (notification, expect) = notification();
        let call = Call::Notification(notification);
        assert_eq!(serde_json::to_string(&call).unwrap(), expect);
        assert_eq!(serde_json::from_str::<Call>(expect).unwrap(), call);
    }

    #[test]
    fn request_serialization() {
        let (call, call_expect) = method_call();
        let call_request = Request::Single(Call::MethodCall(call.clone()));
        assert_eq!(serde_json::to_string(&call_request).unwrap(), call_expect);
        assert_eq!(
            serde_json::from_str::<Request>(call_expect).unwrap(),
            call_request
        );

        let (notification, notification_expect) = notification();
        let notification_request = Request::Single(Call::Notification(notification.clone()));
        assert_eq!(
            serde_json::to_string(&notification_request).unwrap(),
            notification_expect
        );
        assert_eq!(
            serde_json::from_str::<Request>(notification_expect).unwrap(),
            notification_request
        );

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

    #[test]
    fn invalid_request() {
        let cases = vec![
            (
                r#"{"id":1,"method":"foo","params":[1,true],"unknown":[]}"#,
                Request::Single(Call::Invalid { id: Id::Num(1) }),
            ),
            (
                r#"{"method":"foo","params":[1,true],"unknown":[]}"#,
                Request::Single(Call::Invalid { id: Id::Null }),
            ),
            (
                r#"{"unknown":[]}"#,
                Request::Single(Call::Invalid { id: Id::Null }),
            ),
        ];

        for (case, expect) in cases {
            let request = serde_json::from_str::<Request>(case).unwrap();
            assert_eq!(request, expect);
        }
    }
}
