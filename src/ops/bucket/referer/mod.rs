//! Bucket hotlink-protection (Referer) operations.

mod get_bucket_referer;
mod put_bucket_referer;

pub use get_bucket_referer::*;
pub use put_bucket_referer::*;

pub trait BucketRefererOperations: PutBucketRefererOps + GetBucketRefererOps {}
impl<T> BucketRefererOperations for T where T: PutBucketRefererOps + GetBucketRefererOps {}
