//! A set of types for representing JSON-RPC requests and responses as defined in
//! the [specification](https://www.jsonrpc.org/specification).
//!
//! # Usage
//!
//! ## Creates JSON-RPC v1 request
//!
//! ```rust
//! # use jsonrpc_types::{Call, Id, MethodCall, Notification, Params, Request};
//! // Creates a JSON-RPC v1 notification request
//! let notification = Notification {
//!     jsonrpc: None,
//!     method: "foo".into(),
//!     params: Some(Params::Array(vec![])),
//! };
//! let notification_req = Request::Single(Call::Notification(notification.clone()));
//! assert_eq!(
//!     serde_json::to_string(&notification_req).unwrap(),
//!     r#"{"method":"foo","params":[],"id":null}"#
//! );
//!
//! // Creates a JSON-RPC v1 method call request
//! let method_call = MethodCall {
//!     jsonrpc: None,
//!     method: "foo".into(),
//!     params: Some(Params::Array(vec![])),
//!     id: Id::Num(1),
//! };
//! let method_call_req = Request::Single(Call::MethodCall(method_call));
//! assert_eq!(
//!     serde_json::to_string(&method_call_req).unwrap(),
//!     r#"{"method":"foo","params":[],"id":1}"#
//! );
//!
//! // Creates a JSON-RPC v1 batch request
//! let batch_request = Request::Batch(vec![
//!     Call::MethodCall(MethodCall {
//!         jsonrpc: None,
//!         method: "foo".into(),
//!         params: Some(Params::Array(vec![])),
//!         id: Id::Num(1),
//!     }),
//!     Call::MethodCall(MethodCall {
//!         jsonrpc: None,
//!         method: "bar".into(),
//!         params: Some(Params::Array(vec![])),
//!         id: Id::Num(2),
//!     }),
//! ]);
//! assert_eq!(
//!     serde_json::to_string(&batch_request).unwrap(),
//!     r#"[{"method":"foo","params":[],"id":1},{"method":"bar","params":[],"id":2}]"#
//! );
//! ```
//!
//! ## Creates JSON-RPC v2 request
//!
//! ```rust
//! # use jsonrpc_types::{Id, Version, Params, MethodCall, Notification, Call, Request};
//! // Creates a JSON-RPC v2 notification request
//! let notification = Notification {
//!     jsonrpc: Some(Version::V2_0),
//!     method: "foo".into(),
//!     params: Some(Params::Array(vec![])),
//! };
//! let notification_req = Request::Single(Call::Notification(notification.clone()));
//! assert_eq!(
//!     serde_json::to_string(&notification_req).unwrap(),
//!     r#"{"jsonrpc":"2.0","method":"foo","params":[]}"#
//! );
//!
//! // Creates a JSON-RPC v2 method call request
//! let method_call = MethodCall {
//!     jsonrpc: Some(Version::V2_0),
//!     method: "foo".into(),
//!     params: Some(Params::Array(vec![])),
//!     id: Id::Num(1),
//! };
//! let method_call_req = Request::Single(Call::MethodCall(method_call));
//! assert_eq!(
//!     serde_json::to_string(&method_call_req).unwrap(),
//!     r#"{"jsonrpc":"2.0","method":"foo","params":[],"id":1}"#
//! );
//!
//! // Creates a JSON-RPC v2 batch request
//! let batch_request = Request::Batch(vec![
//!     Call::MethodCall(MethodCall {
//!         jsonrpc: Some(Version::V2_0),
//!         method: "foo".into(),
//!         params: Some(Params::Array(vec![])),
//!         id: Id::Num(1),
//!     }),
//!     Call::MethodCall(MethodCall {
//!         jsonrpc: Some(Version::V2_0),
//!         method: "bar".into(),
//!         params: Some(Params::Array(vec![])),
//!         id: Id::Num(2),
//!     }),
//! ]);
//! assert_eq!(
//!     serde_json::to_string(&batch_request).unwrap(),
//!     r#"[{"jsonrpc":"2.0","method":"foo","params":[],"id":1},{"jsonrpc":"2.0","method":"bar","params":[],"id":2}]"#
//! );
//! ```
//!
//! ## Creates JSON-RPC v1 response
//!
//! ```rust
//! # use jsonrpc_types::{Id, Value, Error, Response, ResponseOutput, SuccessResponse, FailureResponse};
//! // Creates a JSON-RPC v1 success response
//! let success_response = SuccessResponse {
//!     jsonrpc: None,
//!     result: Value::Bool(true),
//!     id: Id::Num(1),
//! };
//! let response1 = Response::Single(ResponseOutput::Success(success_response.clone()));
//! assert_eq!(
//!     serde_json::to_string(&response1).unwrap(),
//!     r#"{"result":true,"error":null,"id":1}"#
//! );
//!
//! // Creates a JSON-RPC v1 failure response
//! let failure_response = FailureResponse {
//!     jsonrpc: None,
//!     error: Error::invalid_request(),
//!     id: Id::Num(2),
//! };
//! let response2 = Response::Single(ResponseOutput::Failure(failure_response.clone()));
//! assert_eq!(
//!     serde_json::to_string(&response2).unwrap(),
//!     r#"{"error":{"code":-32600,"message":"Invalid request"},"result":null,"id":2}"#
//! );
//!
//! // Creates a JSON-RPC v1 batch response
//! let batch_response = Response::Batch(vec![
//!     ResponseOutput::Success(success_response),
//!     ResponseOutput::Failure(failure_response)
//! ]);
//! assert_eq!(
//!     serde_json::to_string(&batch_response).unwrap(),
//!     r#"[{"result":true,"error":null,"id":1},{"error":{"code":-32600,"message":"Invalid request"},"result":null,"id":2}]"#
//! );
//! ```
//!
//! ## Creates JSON-RPC v2 response
//!
//! ```rust
//! # use jsonrpc_types::{Id, Version, Value, Error, Response, ResponseOutput, SuccessResponse, FailureResponse};
//! // Creates a JSON-RPC v2 success response
//! let success_response = SuccessResponse {
//!     jsonrpc: Some(Version::V2_0),
//!     result: Value::Bool(true),
//!     id: Id::Num(1),
//! };
//! let response1 = Response::Single(ResponseOutput::Success(success_response.clone()));
//! assert_eq!(
//!     serde_json::to_string(&response1).unwrap(),
//!     r#"{"jsonrpc":"2.0","result":true,"id":1}"#
//! );
//!
//! // Creates a JSON-RPC v2 failure response
//! let failure_response = FailureResponse {
//!     jsonrpc: Some(Version::V2_0),
//!     error: Error::invalid_request(),
//!     id: Id::Num(2),
//! };
//! let response2 = Response::Single(ResponseOutput::Failure(failure_response.clone()));
//! assert_eq!(
//!     serde_json::to_string(&response2).unwrap(),
//!     r#"{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":2}"#
//! );
//!
//! // Creates a JSON-RPC v2 batch response
//! let batch_response = Response::Batch(vec![
//!     ResponseOutput::Success(success_response),
//!     ResponseOutput::Failure(failure_response)
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
pub use serde_json::Value;

mod id;
mod request;
mod response;
mod version;

pub use self::{
    id::Id,
    request::{Call, MethodCall, Notification, Params, Request},
    response::{Error, ErrorCode, FailureResponse, Response, ResponseOutput, SuccessResponse},
    version::Version,
};
