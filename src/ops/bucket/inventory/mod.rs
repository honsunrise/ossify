//! Bucket inventory operations.

mod delete_bucket_inventory;
mod get_bucket_inventory;
mod list_bucket_inventory;
mod put_bucket_inventory;

pub use delete_bucket_inventory::*;
pub use get_bucket_inventory::*;
pub use list_bucket_inventory::*;
pub use put_bucket_inventory::*;

pub trait BucketInventoryOperations:
    PutBucketInventoryOps + GetBucketInventoryOps + ListBucketInventoryOps + DeleteBucketInventoryOps
{
}
impl<T> BucketInventoryOperations for T where
    T: PutBucketInventoryOps + GetBucketInventoryOps + ListBucketInventoryOps + DeleteBucketInventoryOps
{
}
