//! Bucket image style operations.

mod delete_style;
mod get_style;
mod list_style;
mod put_style;

pub use delete_style::*;
pub use get_style::*;
pub use list_style::*;
pub use put_style::*;

pub trait BucketStyleOperations: PutStyleOps + GetStyleOps + ListStyleOps + DeleteStyleOps {}
impl<T> BucketStyleOperations for T where T: PutStyleOps + GetStyleOps + ListStyleOps + DeleteStyleOps {}
