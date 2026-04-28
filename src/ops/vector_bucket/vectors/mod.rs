//! Vector data-plane operations: Put / Get / List / Delete / Query.
//!
//! All five APIs share the same transport pattern: `POST /?<sub-resource>`
//! with a JSON request body. The sub-resource names are `putVectors`,
//! `getVectors`, `listVectors`, `deleteVectors`, and `queryVectors`.

mod delete_vectors;
mod get_vectors;
mod list_vectors;
mod put_vectors;
mod query_vectors;

pub use delete_vectors::*;
pub use get_vectors::*;
pub use list_vectors::*;
pub use put_vectors::*;
pub use query_vectors::*;
