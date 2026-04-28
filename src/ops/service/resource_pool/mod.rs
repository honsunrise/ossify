//! Account-level Resource Pool QoS operations.
//!
//! All APIs in this module are account-level (USE_BUCKET=false). The module
//! covers three sub-groups:
//!
//! - Resource-pool basics: ListResourcePools, GetResourcePoolInfo,
//!   ListResourcePoolBuckets.
//! - Resource-pool per-requester QoS: Put/Get/List/Delete.
//! - Resource-pool bucket-group QoS: ListResourcePoolBucketGroups and
//!   Put/Get/List/Delete ResourcePoolBucketGroupQoSInfo.
//!
//! The bucket-to-group assignment itself is a bucket-level API and lives in
//! `ops::bucket::resource_pool_bucket_group::PutBucketResourcePoolBucketGroup`.

mod delete_resource_pool_bucket_group_qos_info;
mod delete_resource_pool_requester_qos_info;
mod get_resource_pool_bucket_group_qos_info;
mod get_resource_pool_info;
mod get_resource_pool_requester_qos_info;
mod list_resource_pool_bucket_group_qos_infos;
mod list_resource_pool_bucket_groups;
mod list_resource_pool_buckets;
mod list_resource_pool_requester_qos_infos;
mod list_resource_pools;
mod put_resource_pool_bucket_group_qos_info;
mod put_resource_pool_requester_qos_info;

pub use delete_resource_pool_bucket_group_qos_info::*;
pub use delete_resource_pool_requester_qos_info::*;
pub use get_resource_pool_bucket_group_qos_info::*;
pub use get_resource_pool_info::*;
pub use get_resource_pool_requester_qos_info::*;
pub use list_resource_pool_bucket_group_qos_infos::*;
pub use list_resource_pool_bucket_groups::*;
pub use list_resource_pool_buckets::*;
pub use list_resource_pool_requester_qos_infos::*;
pub use list_resource_pools::*;
pub use put_resource_pool_bucket_group_qos_info::*;
pub use put_resource_pool_requester_qos_info::*;

pub trait ResourcePoolOperations:
    ListResourcePoolsOps
    + GetResourcePoolInfoOps
    + ListResourcePoolBucketsOps
    + PutResourcePoolRequesterQoSInfoOps
    + GetResourcePoolRequesterQoSInfoOps
    + ListResourcePoolRequesterQoSInfosOps
    + DeleteResourcePoolRequesterQoSInfoOps
    + ListResourcePoolBucketGroupsOps
    + PutResourcePoolBucketGroupQoSInfoOps
    + GetResourcePoolBucketGroupQoSInfoOps
    + ListResourcePoolBucketGroupQoSInfosOps
    + DeleteResourcePoolBucketGroupQoSInfoOps
{
}
impl<T> ResourcePoolOperations for T where
    T: ListResourcePoolsOps
        + GetResourcePoolInfoOps
        + ListResourcePoolBucketsOps
        + PutResourcePoolRequesterQoSInfoOps
        + GetResourcePoolRequesterQoSInfoOps
        + ListResourcePoolRequesterQoSInfosOps
        + DeleteResourcePoolRequesterQoSInfoOps
        + ListResourcePoolBucketGroupsOps
        + PutResourcePoolBucketGroupQoSInfoOps
        + GetResourcePoolBucketGroupQoSInfoOps
        + ListResourcePoolBucketGroupQoSInfosOps
        + DeleteResourcePoolBucketGroupQoSInfoOps
{
}
