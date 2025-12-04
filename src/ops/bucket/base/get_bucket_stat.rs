use std::future::Future;

use http::Method;
use serde::Deserialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// Bucket statistics data. All statistical items are counted in bytes
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketStat {
    pub storage: u64,
    pub object_count: u64,
    pub multipart_upload_count: u64,
    pub live_channel_count: u64,
    /// Timestamp of when the storage information was obtained, in seconds.
    pub last_modified_time: u64,
    pub standard_storage: u64,
    pub standard_object_count: u64,
    pub infrequent_access_storage: u64,
    pub infrequent_access_real_storage: u64,
    pub infrequent_access_object_count: u64,
    pub archive_storage: u64,
    pub archive_real_storage: u64,
    pub archive_object_count: u64,
    pub cold_archive_storage: u64,
    pub cold_archive_real_storage: u64,
    pub cold_archive_object_count: u64,
    pub deep_cold_archive_storage: u64,
    pub deep_cold_archive_real_storage: u64,
    pub deep_cold_archive_object_count: u64,
}

/// Get bucket statistics operation
pub struct GetBucketStat {}

impl Ops for GetBucketStat {
    type Response = BodyResponseProcessor<BucketStat>;
    type Body = NoneBody;
    type Query = ();

    fn prepare(self) -> Result<Prepared> {
        Ok(Prepared {
            method: Method::GET,
            ..Default::default()
        })
    }
}

pub trait GetBucketStatOps {
    /// Get bucket statistics data
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketstat>
    fn get_bucket_stat(&self) -> impl Future<Output = Result<BucketStat>>;
}

impl GetBucketStatOps for Client {
    async fn get_bucket_stat(&self) -> Result<BucketStat> {
        let ops = GetBucketStat {};
        self.request(ops).await
    }
}
