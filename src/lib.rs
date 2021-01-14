//! A set of types for representing JSON-RPC requests and responses as defined in
//! the [specification](https://www.jsonrpc.org/specification).

#![deny(unused_imports)]
#![deny(missing_docs)]

// Re-exports
pub use serde_json::Value;

mod error;
mod id;
mod params;
mod request;
mod response;
mod version;

pub use self::error::{Error, ErrorCode};
pub use self::id::Id;
pub use self::params::Params;
pub use self::request::{Call, MethodCall, Notification, Request};
pub use self::response::{FailureResponse, Response, ResponseOutput, SuccessResponse};
pub use self::version::Version;
