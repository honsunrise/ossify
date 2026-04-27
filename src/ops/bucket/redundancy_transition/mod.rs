//! Bucket data redundancy transition operations.

mod create_bucket_data_redundancy_transition;
mod delete_bucket_data_redundancy_transition;
mod get_bucket_data_redundancy_transition;
mod list_bucket_data_redundancy_transition;

pub use create_bucket_data_redundancy_transition::*;
pub use delete_bucket_data_redundancy_transition::*;
pub use get_bucket_data_redundancy_transition::*;
pub use list_bucket_data_redundancy_transition::*;

pub trait BucketDataRedundancyTransitionOperations:
    CreateBucketDataRedundancyTransitionOps
    + GetBucketDataRedundancyTransitionOps
    + DeleteBucketDataRedundancyTransitionOps
    + ListBucketDataRedundancyTransitionOps
{
}
impl<T> BucketDataRedundancyTransitionOperations for T where
    T: CreateBucketDataRedundancyTransitionOps
        + GetBucketDataRedundancyTransitionOps
        + DeleteBucketDataRedundancyTransitionOps
        + ListBucketDataRedundancyTransitionOps
{
}
