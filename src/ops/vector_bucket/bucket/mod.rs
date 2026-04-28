//! Vector bucket lifecycle operations: Put / Get / List / Delete.

mod delete_vector_bucket;
mod get_vector_bucket;
mod list_vector_buckets;
mod put_vector_bucket;

pub use delete_vector_bucket::*;
pub use get_vector_bucket::*;
pub use list_vector_buckets::*;
pub use put_vector_bucket::*;
