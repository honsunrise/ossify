//! Bucket server-side encryption operations.

mod delete_bucket_encryption;
mod get_bucket_encryption;
mod put_bucket_encryption;

pub use delete_bucket_encryption::*;
pub use get_bucket_encryption::*;
pub use put_bucket_encryption::*;

pub trait BucketEncryptionOperations:
    PutBucketEncryptionOps + GetBucketEncryptionOps + DeleteBucketEncryptionOps
{
}
impl<T> BucketEncryptionOperations for T where
    T: PutBucketEncryptionOps + GetBucketEncryptionOps + DeleteBucketEncryptionOps
{
}
