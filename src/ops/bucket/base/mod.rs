mod common;
mod delete_bucket;
mod get_bucket_info;
mod get_bucket_location;
mod get_bucket_stat;
mod list_objects_v1;
mod list_objects_v2;
mod put_bucket;

pub use common::*;
pub use delete_bucket::*;
pub use get_bucket_info::*;
pub use get_bucket_location::*;
pub use get_bucket_stat::*;
pub use list_objects_v1::*;
pub use list_objects_v2::*;
pub use put_bucket::*;

// Combined trait that includes all bucket base operations
pub trait BucketOperations:
    DeleteBucketOps
    + GetBucketInfoOps
    + GetBucketLocationOps
    + GetBucketStatOps
    + ListObjectsOps
    + ListObjectsV1Ops
    + PutBucketOps
{
}

// Blanket implementation for any type that implements all the individual traits
impl<T> BucketOperations for T where
    T: DeleteBucketOps
        + GetBucketInfoOps
        + GetBucketLocationOps
        + GetBucketStatOps
        + ListObjectsOps
        + ListObjectsV1Ops
        + PutBucketOps
{
}
