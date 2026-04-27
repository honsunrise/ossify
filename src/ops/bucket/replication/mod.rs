//! Bucket replication (CRR/SRR) operations.

mod delete_bucket_replication;
mod get_bucket_replication;
mod get_bucket_replication_location;
mod get_bucket_replication_progress;
mod put_bucket_replication;
mod put_bucket_rtc;

pub use delete_bucket_replication::*;
pub use get_bucket_replication::*;
pub use get_bucket_replication_location::*;
pub use get_bucket_replication_progress::*;
pub use put_bucket_replication::*;
pub use put_bucket_rtc::*;

pub trait BucketReplicationOperations:
    PutBucketReplicationOps
    + PutBucketRtcOps
    + GetBucketReplicationOps
    + GetBucketReplicationLocationOps
    + GetBucketReplicationProgressOps
    + DeleteBucketReplicationOps
{
}
impl<T> BucketReplicationOperations for T where
    T: PutBucketReplicationOps
        + PutBucketRtcOps
        + GetBucketReplicationOps
        + GetBucketReplicationLocationOps
        + GetBucketReplicationProgressOps
        + DeleteBucketReplicationOps
{
}
