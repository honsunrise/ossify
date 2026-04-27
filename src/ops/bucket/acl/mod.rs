//! Bucket access control list (ACL) operations.

mod get_bucket_acl;
mod put_bucket_acl;

pub use get_bucket_acl::*;
pub use put_bucket_acl::*;

pub trait BucketAclOperations: GetBucketAclOps + PutBucketAclOps {}
impl<T> BucketAclOperations for T where T: GetBucketAclOps + PutBucketAclOps {}
