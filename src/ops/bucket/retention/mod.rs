//! Bucket retention (WORM) policy operations.
//!
//! OSS supports Write Once Read Many (WORM) retention policies. The policy
//! goes through these states:
//!
//! 1. [`InitiateBucketWorm`]: create the policy. Returns a `WormId`. The policy
//!    is `InProgress` for 24 hours.
//! 2. [`AbortBucketWorm`]: cancel an unlocked (InProgress) policy.
//! 3. [`CompleteBucketWorm`]: lock the policy so it can no longer be deleted.
//! 4. [`ExtendBucketWorm`]: extend the retention period of a locked policy.
//! 5. [`GetBucketWorm`]: query the current policy.
//!
//! See the top-level "Retention policy" API index at
//! <https://www.alibabacloud.com/help/en/oss/developer-reference/retention-policy/>.

mod abort_bucket_worm;
mod complete_bucket_worm;
mod extend_bucket_worm;
mod get_bucket_worm;
mod initiate_bucket_worm;

pub use abort_bucket_worm::*;
pub use complete_bucket_worm::*;
pub use extend_bucket_worm::*;
pub use get_bucket_worm::*;
pub use initiate_bucket_worm::*;

/// Aggregate trait for all WORM retention-policy operations.
pub trait BucketWormOperations:
    InitiateBucketWormOps + AbortBucketWormOps + CompleteBucketWormOps + ExtendBucketWormOps + GetBucketWormOps
{
}

impl<T> BucketWormOperations for T where
    T: InitiateBucketWormOps
        + AbortBucketWormOps
        + CompleteBucketWormOps
        + ExtendBucketWormOps
        + GetBucketWormOps
{
}
