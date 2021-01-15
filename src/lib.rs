//! A set of types for representing JSON-RPC requests and responses as defined in
//! the [specification](https://www.jsonrpc.org/specification).

#![deny(unused_imports)]
#![deny(missing_docs)]

// Re-exports
pub use serde_json::Value;

pub use self::id::Id;
pub use self::request::{Call, MethodCall, Notification, Params, Request};
pub use self::response::{
    Error, ErrorCode, FailureResponse, Response, ResponseOutput, SuccessResponse,
};
pub use self::version::Version;

mod id;
mod request;
mod response;
mod version;
