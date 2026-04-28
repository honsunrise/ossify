//! Bucket QoS (total bandwidth cap per bucket) operations.
//!
//! This controls the whole bucket's bandwidth, not per-requester limits.
//! For per-requester caps see the `requester_qos` module.

mod delete_bucket_qos_info;
mod get_bucket_qos_info;
mod put_bucket_qos_info;

pub use delete_bucket_qos_info::*;
pub use get_bucket_qos_info::*;
pub use put_bucket_qos_info::*;

pub trait BucketQoSOperations: PutBucketQoSInfoOps + GetBucketQoSInfoOps + DeleteBucketQoSInfoOps {}
impl<T> BucketQoSOperations for T where T: PutBucketQoSInfoOps + GetBucketQoSInfoOps + DeleteBucketQoSInfoOps {}
