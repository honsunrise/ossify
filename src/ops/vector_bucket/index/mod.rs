//! Vector index operations: Put / Get / List / Delete.
//!
//! All four APIs are `POST /?<sub-resource>` with a JSON request body
//! carrying the operation parameters, and a JSON response body where
//! applicable.

mod delete_vector_index;
mod get_vector_index;
mod list_vector_indexes;
mod put_vector_index;

pub use delete_vector_index::*;
pub use get_vector_index::*;
pub use list_vector_indexes::*;
pub use put_vector_index::*;
