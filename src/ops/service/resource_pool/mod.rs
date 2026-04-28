//! Account-level Resource Pool QoS operations.
//!
//! All seven APIs in this module are account-level (USE_BUCKET=false):
//!
//! - ListResourcePools
//! - GetResourcePoolInfo
//! - ListResourcePoolBuckets
//! - PutResourcePoolRequesterQoSInfo
//! - GetResourcePoolRequesterQoSInfo
//! - ListResourcePoolRequesterQoSInfos
//! - DeleteResourcePoolRequesterQoSInfo
//!
//! A "resource pool" is a quota container provisioned by Alibaba Cloud for
//! approved customers; clients cannot create or delete the pools themselves,
//! only read pool metadata and configure per-requester QoS against them.

mod delete_resource_pool_requester_qos_info;
mod get_resource_pool_info;
mod get_resource_pool_requester_qos_info;
mod list_resource_pool_buckets;
mod list_resource_pool_requester_qos_infos;
mod list_resource_pools;
mod put_resource_pool_requester_qos_info;

pub use delete_resource_pool_requester_qos_info::*;
pub use get_resource_pool_info::*;
pub use get_resource_pool_requester_qos_info::*;
pub use list_resource_pool_buckets::*;
pub use list_resource_pool_requester_qos_infos::*;
pub use list_resource_pools::*;
pub use put_resource_pool_requester_qos_info::*;

pub trait ResourcePoolOperations:
    ListResourcePoolsOps
    + GetResourcePoolInfoOps
    + ListResourcePoolBucketsOps
    + PutResourcePoolRequesterQoSInfoOps
    + GetResourcePoolRequesterQoSInfoOps
    + ListResourcePoolRequesterQoSInfosOps
    + DeleteResourcePoolRequesterQoSInfoOps
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
{
}
