//! Bucket-scoped Anti-DDoS (DDoS protection) operations.
//!
//! The three account-level APIs (Init/Update/Get UserAntiDDosInfo) live under
//! [`crate::ops::service`] because they do not target a bucket.

mod init_bucket_anti_ddos_info;
mod list_bucket_anti_ddos_info;
mod update_bucket_anti_ddos_info;

pub use init_bucket_anti_ddos_info::*;
pub use list_bucket_anti_ddos_info::*;
pub use update_bucket_anti_ddos_info::*;

pub trait BucketAntiDdosOperations:
    InitBucketAntiDDosInfoOps + UpdateBucketAntiDDosInfoOps + ListBucketAntiDDosInfoOps
{
}

impl<T> BucketAntiDdosOperations for T where
    T: InitBucketAntiDDosInfoOps + UpdateBucketAntiDDosInfoOps + ListBucketAntiDDosInfoOps
{
}
