//! Bucket static-website-hosting operations.

mod delete_bucket_website;
mod get_bucket_website;
mod put_bucket_website;

pub use delete_bucket_website::*;
pub use get_bucket_website::*;
pub use put_bucket_website::*;

pub trait BucketWebsiteOperations:
    PutBucketWebsiteOps + GetBucketWebsiteOps + DeleteBucketWebsiteOps
{
}
impl<T> BucketWebsiteOperations for T where
    T: PutBucketWebsiteOps + GetBucketWebsiteOps + DeleteBucketWebsiteOps
{
}
