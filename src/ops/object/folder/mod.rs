//! Folder / directory management operations (Hierarchical Namespace Service).
//!
//! These APIs are available only on buckets that have the hierarchical
//! namespace (HNS) feature enabled. OSS models HNS directories as distinct
//! resources (as opposed to the `prefix/` pseudo-folder model).

mod create_directory;
mod delete_directory;
mod rename;

pub use create_directory::*;
pub use delete_directory::*;
pub use rename::*;
