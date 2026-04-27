//! OSS Accelerator (DataAccelerator) bucket operations.

mod delete_bucket_data_accelerator;
mod get_bucket_data_accelerator;
mod put_bucket_data_accelerator;

pub use delete_bucket_data_accelerator::*;
pub use get_bucket_data_accelerator::*;
pub use put_bucket_data_accelerator::*;

pub trait BucketDataAcceleratorOperations:
    PutBucketDataAcceleratorOps + GetBucketDataAcceleratorOps + DeleteBucketDataAcceleratorOps
{
}
impl<T> BucketDataAcceleratorOperations for T where
    T: PutBucketDataAcceleratorOps + GetBucketDataAcceleratorOps + DeleteBucketDataAcceleratorOps
{
}
