//! Bucket versioning operations.

mod get_bucket_versioning;
mod list_object_versions;
mod put_bucket_versioning;

pub use get_bucket_versioning::*;
pub use list_object_versions::*;
pub use put_bucket_versioning::*;

pub trait BucketVersioningOperations:
    PutBucketVersioningOps + GetBucketVersioningOps + ListObjectVersionsOps
{
}
impl<T> BucketVersioningOperations for T where
    T: PutBucketVersioningOps + GetBucketVersioningOps + ListObjectVersionsOps
{
}
