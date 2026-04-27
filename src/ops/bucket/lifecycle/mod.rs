//! Bucket lifecycle rule operations.
//!
//! Lifecycle rules can change the storage class of objects, delete expired
//! objects, and abort stale multipart uploads. See
//! [`LifecycleConfiguration`](crate::ops::common::LifecycleConfiguration) for
//! the shape of the rule list.

mod delete_bucket_lifecycle;
mod get_bucket_lifecycle;
mod put_bucket_lifecycle;

pub use delete_bucket_lifecycle::*;
pub use get_bucket_lifecycle::*;
pub use put_bucket_lifecycle::*;

pub trait BucketLifecycleOperations:
    PutBucketLifecycleOps + GetBucketLifecycleOps + DeleteBucketLifecycleOps
{
}
impl<T> BucketLifecycleOperations for T where
    T: PutBucketLifecycleOps + GetBucketLifecycleOps + DeleteBucketLifecycleOps
{
}
