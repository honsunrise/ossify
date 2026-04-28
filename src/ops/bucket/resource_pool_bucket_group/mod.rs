//! Bucket-level resource-pool bucket-group operations.
//!
//! Only `PutBucketResourcePoolBucketGroup` is bucket-scoped. The remaining
//! bucket-group APIs (list, QoS read/write/delete) live at account scope
//! under `ops::service::resource_pool`.

mod put_bucket_resource_pool_bucket_group;

pub use put_bucket_resource_pool_bucket_group::*;

pub trait BucketResourcePoolBucketGroupOperations: PutBucketResourcePoolBucketGroupOps {}
impl<T> BucketResourcePoolBucketGroupOperations for T where T: PutBucketResourcePoolBucketGroupOps {}
