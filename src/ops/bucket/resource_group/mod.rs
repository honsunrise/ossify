//! Bucket resource-group operations.

mod get_bucket_resource_group;
mod put_bucket_resource_group;

pub use get_bucket_resource_group::*;
pub use put_bucket_resource_group::*;

pub trait BucketResourceGroupOperations: PutBucketResourceGroupOps + GetBucketResourceGroupOps {}
impl<T> BucketResourceGroupOperations for T where T: PutBucketResourceGroupOps + GetBucketResourceGroupOps {}
