//! Bucket CORS operations.

mod delete_bucket_cors;
mod get_bucket_cors;
mod options_object;
mod put_bucket_cors;

pub use delete_bucket_cors::*;
pub use get_bucket_cors::*;
pub use options_object::*;
pub use put_bucket_cors::*;

pub trait BucketCorsOperations:
    PutBucketCorsOps + GetBucketCorsOps + DeleteBucketCorsOps + OptionsObjectOps
{
}
impl<T> BucketCorsOperations for T where
    T: PutBucketCorsOps + GetBucketCorsOps + DeleteBucketCorsOps + OptionsObjectOps
{
}
