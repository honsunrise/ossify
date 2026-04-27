mod describe_regions;
mod list_buckets;
mod list_user_data_redundancy_transition;

pub use describe_regions::*;
pub use list_buckets::*;
pub use list_user_data_redundancy_transition::*;

// Aggregated operations trait (blanket-implemented for any type that
// implements all individual service-level ops traits).
pub trait ServiceOperations:
    DescribeRegionsOps + ListBucketsOps + ListUserDataRedundancyTransitionOps
{
}

impl<T> ServiceOperations for T where
    T: DescribeRegionsOps + ListBucketsOps + ListUserDataRedundancyTransitionOps
{
}
