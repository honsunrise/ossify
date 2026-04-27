//! Bucket access-tracking (access monitor) operations.

mod get_bucket_access_monitor;
mod put_bucket_access_monitor;

pub use get_bucket_access_monitor::*;
pub use put_bucket_access_monitor::*;

pub trait BucketAccessMonitorOperations: PutBucketAccessMonitorOps + GetBucketAccessMonitorOps {}
impl<T> BucketAccessMonitorOperations for T where T: PutBucketAccessMonitorOps + GetBucketAccessMonitorOps {}
