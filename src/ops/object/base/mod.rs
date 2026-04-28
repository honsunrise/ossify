mod append_object;
mod callback;
mod clean_restored_object;
mod copy_object;
mod delete_multiple_objects;
mod delete_object;
mod get_object;
mod get_object_meta;
mod head_object;
mod post_object;
mod put_object;
mod restore_object;
mod seal_append_object;

// Re-export canonical shared types for backwards compatibility.
pub use append_object::*;
pub use callback::*;
pub use clean_restored_object::*;
pub use copy_object::*;
pub use delete_multiple_objects::*;
pub use delete_object::*;
pub use get_object::*;
pub use get_object_meta::*;
pub use head_object::*;
pub use post_object::*;
pub use put_object::*;
pub use restore_object::*;
pub use seal_append_object::*;

pub use crate::ops::common::{ServerSideEncryption, StorageClass};
