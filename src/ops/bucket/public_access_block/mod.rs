//! Bucket- and access-point-level Block Public Access operations.
//!
//! The three global (user-level) PublicAccessBlock APIs live under
//! [`crate::ops::service`] because they do not target a bucket.

mod delete_access_point_public_access_block;
mod delete_bucket_public_access_block;
mod get_access_point_public_access_block;
mod get_bucket_public_access_block;
mod put_access_point_public_access_block;
mod put_bucket_public_access_block;

pub use delete_access_point_public_access_block::*;
pub use delete_bucket_public_access_block::*;
pub use get_access_point_public_access_block::*;
pub use get_bucket_public_access_block::*;
pub use put_access_point_public_access_block::*;
pub use put_bucket_public_access_block::*;

pub trait BucketPublicAccessBlockOperations:
    PutBucketPublicAccessBlockOps
    + GetBucketPublicAccessBlockOps
    + DeleteBucketPublicAccessBlockOps
    + PutAccessPointPublicAccessBlockOps
    + GetAccessPointPublicAccessBlockOps
    + DeleteAccessPointPublicAccessBlockOps
{
}

impl<T> BucketPublicAccessBlockOperations for T where
    T: PutBucketPublicAccessBlockOps
        + GetBucketPublicAccessBlockOps
        + DeleteBucketPublicAccessBlockOps
        + PutAccessPointPublicAccessBlockOps
        + GetAccessPointPublicAccessBlockOps
        + DeleteAccessPointPublicAccessBlockOps
{
}
