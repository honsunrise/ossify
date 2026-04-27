//! Shared OSS types used across multiple API operations.
//!
//! As the SDK grows to cover the entire OSS API surface, many request/response
//! payloads reference the same XML element types (for example `StorageClass`,
//! `ServerSideEncryption`, or `Owner`). Keeping them in one place avoids drift
//! between duplicate definitions and makes them easy to reuse in future
//! operations.

mod encoding_type;
mod encryption;
mod object_type;
mod owner;
mod storage_class;

pub use encoding_type::EncodingType;
pub use encryption::ServerSideEncryption;
pub use object_type::ObjectType;
pub use owner::Owner;
pub use storage_class::{DataRedundancyType, StorageClass};
