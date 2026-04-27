//! Bucket policy (JSON-based authorization policy) operations.

mod delete_bucket_policy;
mod get_bucket_policy;
mod get_bucket_policy_status;
mod put_bucket_policy;

pub use delete_bucket_policy::*;
pub use get_bucket_policy::*;
pub use get_bucket_policy_status::*;
pub use put_bucket_policy::*;

pub trait BucketPolicyOperations:
    PutBucketPolicyOps + GetBucketPolicyOps + GetBucketPolicyStatusOps + DeleteBucketPolicyOps
{
}
impl<T> BucketPolicyOperations for T where
    T: PutBucketPolicyOps + GetBucketPolicyOps + GetBucketPolicyStatusOps + DeleteBucketPolicyOps
{
}
