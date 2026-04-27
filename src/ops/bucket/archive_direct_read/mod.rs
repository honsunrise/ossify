//! Bucket real-time Archive access operations.

mod get_bucket_archive_direct_read;
mod put_bucket_archive_direct_read;

pub use get_bucket_archive_direct_read::*;
pub use put_bucket_archive_direct_read::*;

pub trait BucketArchiveDirectReadOperations:
    PutBucketArchiveDirectReadOps + GetBucketArchiveDirectReadOps
{
}
impl<T> BucketArchiveDirectReadOperations for T where
    T: PutBucketArchiveDirectReadOps + GetBucketArchiveDirectReadOps
{
}
