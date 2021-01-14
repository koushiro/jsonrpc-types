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
