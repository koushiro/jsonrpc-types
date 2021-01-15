//! A set of types for representing JSON-RPC requests and responses as defined in
//! the [specification](https://www.jsonrpc.org/specification).

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
