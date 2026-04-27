//! Bucket pay-by-requester operations.

mod get_bucket_request_payment;
mod put_bucket_request_payment;

pub use get_bucket_request_payment::*;
pub use put_bucket_request_payment::*;

pub trait BucketRequestPaymentOperations: PutBucketRequestPaymentOps + GetBucketRequestPaymentOps {}
impl<T> BucketRequestPaymentOperations for T where T: PutBucketRequestPaymentOps + GetBucketRequestPaymentOps {}
