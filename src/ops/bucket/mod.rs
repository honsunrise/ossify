pub mod base;
pub mod data_indexing;

use std::future::Future;

// Re-export all base module types and traits
pub use base::*;

use crate::Client;
use crate::error::Result;

// =============================================================================
// Trait definition for backward compatibility
// =============================================================================

/// Main bucket operations trait (for backward compatibility)
/// This trait re-exports all operations from the base module
pub trait BucketOperations {
    /// Create a new bucket
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucket>
    fn put_bucket(
        &self,
        config: PutBucketConfiguration,
        options: Option<PutBucketOptions>,
    ) -> impl Future<Output = Result<()>>;

    /// Get bucket information
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketinfo>
    fn get_bucket_info(&self) -> impl Future<Output = Result<BucketDetail>>;

    /// Get bucket location
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketlocation>
    fn get_bucket_location(&self) -> impl Future<Output = Result<String>>;

    /// Get bucket statistics data
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketstat>
    fn get_bucket_stat(&self) -> impl Future<Output = Result<BucketStat>>;

    /// List objects in a bucket (V2)
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listobjectsv2>
    fn list_objects(
        &self,
        params: Option<ListObjectsV2Params>,
    ) -> impl Future<Output = Result<ListObjectsResult>>;

    /// Delete a bucket
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucket>
    fn delete_bucket(&self) -> impl Future<Output = Result<()>>;
}

// =============================================================================
// Implementation for Client
// =============================================================================

impl BucketOperations for Client {
    async fn put_bucket(
        &self,
        config: PutBucketConfiguration,
        options: Option<PutBucketOptions>,
    ) -> Result<()> {
        base::PutBucketOps::put_bucket(self, config, options).await
    }

    async fn get_bucket_info(&self) -> Result<BucketDetail> {
        base::GetBucketInfoOps::get_bucket_info(self).await
    }

    async fn get_bucket_location(&self) -> Result<String> {
        base::GetBucketLocationOps::get_bucket_location(self).await
    }

    async fn get_bucket_stat(&self) -> Result<BucketStat> {
        base::GetBucketStatOps::get_bucket_stat(self).await
    }

    async fn list_objects(&self, params: Option<ListObjectsV2Params>) -> Result<ListObjectsResult> {
        base::ListObjectsOps::list_objects(self, params).await
    }

    async fn delete_bucket(&self) -> Result<()> {
        base::DeleteBucketOps::delete_bucket(self).await
    }
}
