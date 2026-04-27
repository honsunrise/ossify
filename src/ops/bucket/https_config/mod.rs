//! Bucket transport-layer-security operations.

mod get_bucket_https_config;
mod put_bucket_https_config;

pub use get_bucket_https_config::*;
pub use put_bucket_https_config::*;

pub trait BucketHttpsConfigOperations: PutBucketHttpsConfigOps + GetBucketHttpsConfigOps {}
impl<T> BucketHttpsConfigOperations for T where T: PutBucketHttpsConfigOps + GetBucketHttpsConfigOps {}
