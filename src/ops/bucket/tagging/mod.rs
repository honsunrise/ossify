//! Bucket tagging operations.

mod delete_bucket_tags;
mod get_bucket_tags;
mod put_bucket_tags;

pub use delete_bucket_tags::*;
pub use get_bucket_tags::*;
pub use put_bucket_tags::*;

pub trait BucketTaggingOperations: PutBucketTagsOps + GetBucketTagsOps + DeleteBucketTagsOps {}
impl<T> BucketTaggingOperations for T where T: PutBucketTagsOps + GetBucketTagsOps + DeleteBucketTagsOps {}
