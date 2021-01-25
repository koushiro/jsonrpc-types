//! A set of types for representing JSON-RPC requests and responses as defined in
//! the [JSON-RPC 1.0 spec](https://www.jsonrpc.org/specification_v1) and
//! [JSON-RPC 2.0 spec](https://www.jsonrpc.org/specification).
//!
//! # Usage
//!
//! ## Creates JSON-RPC 1.0 request
//!
//! ```rust
//! # use jsonrpc_types::{Call, Id, MethodCall, Notification, Params, Request};
//! // Creates a JSON-RPC 1.0 method call request
//! let method_call = MethodCall::new_v1("foo", Params::Array(vec![]), 1.into());
//! let method_call_req = Request::Single(Call::MethodCall(method_call));
//! assert_eq!(
//!     serde_json::to_string(&method_call_req).unwrap(),
//!     r#"{"method":"foo","params":[],"id":1}"#
//! );
//!
//! // Creates a JSON-RPC 1.0 notification request
//! let notification = Notification::new_v1("foo", Params::Array(vec![]));
//! let notification_req = Request::Single(Call::Notification(notification.clone()));
//! assert_eq!(
//!     serde_json::to_string(&notification_req).unwrap(),
//!     r#"{"method":"foo","params":[],"id":null}"#
//! );
//!
//! // Creates a JSON-RPC 1.0 batch request
//! let batch_request = Request::Batch(vec![
//!     Call::MethodCall(MethodCall::new_v1("foo", Params::Array(vec![]), 1.into())),
//!     Call::MethodCall(MethodCall::new_v1("bar", Params::Array(vec![]), 2.into())),
//! ]);
//! assert_eq!(
//!     serde_json::to_string(&batch_request).unwrap(),
//!     r#"[{"method":"foo","params":[],"id":1},{"method":"bar","params":[],"id":2}]"#
//! );
//! ```
//!
//! ## Creates JSON-RPC 1.0 response
//!
//! ```rust
//! # use jsonrpc_types::{Id, JsonValue, Error, Response, Output, Success, Failure};
//! // Creates a JSON-RPC 1.0 success response
//! let success_response = Success::new_v1(true.into(), 1.into());
//! let response1 = Response::Single(Output::Success(success_response.clone()));
//! assert_eq!(
//!     serde_json::to_string(&response1).unwrap(),
//!     r#"{"result":true,"error":null,"id":1}"#
//! );
//!
//! // Creates a JSON-RPC 1.0 failure response
//! let failure_response = Failure::new_v1(Error::invalid_request(), 2.into());
//! let response2 = Response::Single(Output::Failure(failure_response.clone()));
//! assert_eq!(
//!     serde_json::to_string(&response2).unwrap(),
//!     r#"{"error":{"code":-32600,"message":"Invalid request"},"result":null,"id":2}"#
//! );
//!
//! // Creates a JSON-RPC 1.0 batch response
//! let batch_response = Response::Batch(vec![
//!     Output::Success(success_response),
//!     Output::Failure(failure_response)
//! ]);
//! assert_eq!(
//!     serde_json::to_string(&batch_response).unwrap(),
//!     r#"[{"result":true,"error":null,"id":1},{"error":{"code":-32600,"message":"Invalid request"},"result":null,"id":2}]"#
//! );
//! ```
//!
//! ## Creates JSON-RPC 2.0 request
//!
//! ```rust
//! # use jsonrpc_types::{Id, Version, Params, MethodCall, Notification, Call, Request};
//! // Creates a JSON-RPC 2.0 method call request
//! let method_call = MethodCall::new_v2("foo", Some(Params::Array(vec![])), 1.into());
//! let method_call_req = Request::Single(Call::MethodCall(method_call));
//! assert_eq!(
//!     serde_json::to_string(&method_call_req).unwrap(),
//!     r#"{"jsonrpc":"2.0","method":"foo","params":[],"id":1}"#
//! );
//!
//! // Creates a JSON-RPC 2.0 notification request
//! let notification = Notification::new_v2("foo", Some(Params::Array(vec![])));
//! let notification_req = Request::Single(Call::Notification(notification.clone()));
//! assert_eq!(
//!     serde_json::to_string(&notification_req).unwrap(),
//!     r#"{"jsonrpc":"2.0","method":"foo","params":[]}"#
//! );
//!
//! // Creates a JSON-RPC 2.0 batch request
//! let batch_request = Request::Batch(vec![
//!     Call::MethodCall(MethodCall::new_v2("foo", Some(Params::Array(vec![])), 1.into())),
//!     Call::MethodCall(MethodCall::new_v2("bar", Some(Params::Array(vec![])), 2.into())),
//! ]);
//! assert_eq!(
//!     serde_json::to_string(&batch_request).unwrap(),
//!     r#"[{"jsonrpc":"2.0","method":"foo","params":[],"id":1},{"jsonrpc":"2.0","method":"bar","params":[],"id":2}]"#
//! );
//! ```
//!
//! ## Creates JSON-RPC 2.0 response
//!
//! ```rust
//! # use jsonrpc_types::{Id, Version, JsonValue, Error, Response, Output, Success, Failure};
//! // Creates a JSON-RPC 2.0 success response
//! let success = Success::new_v2(true.into(), 1.into());
//! let response1 = Response::Single(Output::Success(success.clone()));
//! assert_eq!(
//!     serde_json::to_string(&response1).unwrap(),
//!     r#"{"jsonrpc":"2.0","result":true,"id":1}"#
//! );
//!
//! // Creates a JSON-RPC 2.0 failure response
//! let failure = Failure::new_v2(Error::invalid_request(), 2.into());
//! let response2 = Response::Single(Output::Failure(failure.clone()));
//! assert_eq!(
//!     serde_json::to_string(&response2).unwrap(),
//!     r#"{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":2}"#
//! );
//!
//! // Creates a JSON-RPC 2.0 batch response
//! let batch_response = Response::Batch(vec![
//!     Output::Success(success),
//!     Output::Failure(failure)
//! ]);
//! assert_eq!(
//!     serde_json::to_string(&batch_response).unwrap(),
//!     r#"[{"jsonrpc":"2.0","result":true,"id":1},{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":2}]"#
//! );
//! ```
//!

#![deny(unused_imports)]
#![deny(missing_docs)]

// Re-exports
pub use serde_json::{Map as JsonMap, Value as JsonValue};

mod id;
mod request;
mod response;
mod version;

pub use self::{
    id::Id,
    request::{Call, MethodCall, Notification, Params, Request},
    response::{Error, ErrorCode, Failure, Output, Response, Success},
    version::Version,
};
