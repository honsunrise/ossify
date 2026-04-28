//! Bucket-level requester QoS (per-UID bandwidth cap) operations.
//!
//! These control how much bandwidth a specific Alibaba Cloud UID is allowed
//! to consume against the current bucket. See the resource-pool requester
//! QoS module for the account-wide / pool-wide variant (which additionally
//! supports QPS limits).

mod delete_bucket_requester_qos_info;
mod get_bucket_requester_qos_info;
mod list_bucket_requester_qos_infos;
mod put_bucket_requester_qos_info;

pub use delete_bucket_requester_qos_info::*;
pub use get_bucket_requester_qos_info::*;
pub use list_bucket_requester_qos_infos::*;
pub use put_bucket_requester_qos_info::*;

pub trait BucketRequesterQoSOperations:
    PutBucketRequesterQoSInfoOps
    + GetBucketRequesterQoSInfoOps
    + ListBucketRequesterQoSInfosOps
    + DeleteBucketRequesterQoSInfoOps
{
}
impl<T> BucketRequesterQoSOperations for T where
    T: PutBucketRequesterQoSInfoOps
        + GetBucketRequesterQoSInfoOps
        + ListBucketRequesterQoSInfosOps
        + DeleteBucketRequesterQoSInfoOps
{
}
