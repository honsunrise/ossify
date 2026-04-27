mod describe_regions;
mod get_user_anti_ddos_info;
mod init_user_anti_ddos_info;
mod list_buckets;
mod list_user_data_redundancy_transition;
mod update_user_anti_ddos_info;

pub use describe_regions::*;
pub use get_user_anti_ddos_info::*;
pub use init_user_anti_ddos_info::*;
pub use list_buckets::*;
pub use list_user_data_redundancy_transition::*;
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
{
}

impl<T> ServiceOperations for T where
    T: DescribeRegionsOps
        + ListBucketsOps
        + ListUserDataRedundancyTransitionOps
        + InitUserAntiDDosInfoOps
        + UpdateUserAntiDDosInfoOps
        + GetUserAntiDDosInfoOps
{
}
