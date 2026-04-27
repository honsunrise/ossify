//! Bucket transfer acceleration operations.

mod get_bucket_transfer_acceleration;
mod put_bucket_transfer_acceleration;

pub use get_bucket_transfer_acceleration::*;
pub use put_bucket_transfer_acceleration::*;

pub trait BucketTransferAccelerationOperations:
    PutBucketTransferAccelerationOps + GetBucketTransferAccelerationOps
{
}
impl<T> BucketTransferAccelerationOperations for T where
    T: PutBucketTransferAccelerationOps + GetBucketTransferAccelerationOps
{
}
