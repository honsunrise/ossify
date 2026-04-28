mod delete_public_access_block;
mod describe_regions;
mod get_public_access_block;
mod get_user_anti_ddos_info;
mod init_user_anti_ddos_info;
mod list_buckets;
mod list_user_data_redundancy_transition;
mod put_public_access_block;
pub mod resource_pool;
mod update_user_anti_ddos_info;

pub use delete_public_access_block::*;
pub use describe_regions::*;
pub use get_public_access_block::*;
pub use get_user_anti_ddos_info::*;
pub use init_user_anti_ddos_info::*;
pub use list_buckets::*;
pub use list_user_data_redundancy_transition::*;
pub use put_public_access_block::*;
pub use resource_pool::*;
pub use update_user_anti_ddos_info::*;

// Aggregated operations trait (blanket-implemented for any type that
// implements all individual service-level ops traits).
pub trait ServiceOperations:
    DescribeRegionsOps
    + ListBucketsOps
    + ListUserDataRedundancyTransitionOps
    + InitUserAntiDDosInfoOps
    + UpdateUserAntiDDosInfoOps
    + GetUserAntiDDosInfoOps
    + PutPublicAccessBlockOps
    + GetPublicAccessBlockOps
    + DeletePublicAccessBlockOps
{
}

impl<T> ServiceOperations for T where
    T: DescribeRegionsOps
        + ListBucketsOps
        + ListUserDataRedundancyTransitionOps
        + InitUserAntiDDosInfoOps
        + UpdateUserAntiDDosInfoOps
        + GetUserAntiDDosInfoOps
        + PutPublicAccessBlockOps
        + GetPublicAccessBlockOps
        + DeletePublicAccessBlockOps
{
}
