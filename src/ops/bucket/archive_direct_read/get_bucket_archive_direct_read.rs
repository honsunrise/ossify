//! GetBucketArchiveDirectRead.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketarchivedirectread>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_archive_direct_read::ArchiveDirectReadConfiguration;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketArchiveDirectReadParams {
    #[serde(rename = "bucketArchiveDirectRead")]
    bucket_archive_direct_read: OnlyKeyField,
}

pub struct GetBucketArchiveDirectRead;

impl Ops for GetBucketArchiveDirectRead {
    type Response = BodyResponseProcessor<ArchiveDirectReadConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketArchiveDirectReadParams;

    fn prepare(self) -> Result<Prepared<GetBucketArchiveDirectReadParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketArchiveDirectReadParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketArchiveDirectReadOps {
    /// Query whether real-time Archive access is enabled for the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketarchivedirectread>
    fn get_bucket_archive_direct_read(&self) -> impl Future<Output = Result<ArchiveDirectReadConfiguration>>;
}

impl GetBucketArchiveDirectReadOps for Client {
    async fn get_bucket_archive_direct_read(&self) -> Result<ArchiveDirectReadConfiguration> {
        self.request(GetBucketArchiveDirectRead).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketArchiveDirectReadParams::default()).unwrap(),
            "bucketArchiveDirectRead"
        );
    }
}
