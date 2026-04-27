//! PutBucketArchiveDirectRead.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketarchivedirectread>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketArchiveDirectReadParams {
    #[serde(rename = "bucketArchiveDirectRead")]
    bucket_archive_direct_read: OnlyKeyField,
}

/// Root `<ArchiveDirectReadConfiguration>` element.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ArchiveDirectReadConfiguration", rename_all = "PascalCase")]
pub struct ArchiveDirectReadConfiguration {
    pub enabled: bool,
}

pub struct PutBucketArchiveDirectRead {
    pub enabled: bool,
}

impl Ops for PutBucketArchiveDirectRead {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<ArchiveDirectReadConfiguration>;
    type Query = PutBucketArchiveDirectReadParams;

    fn prepare(self) -> Result<Prepared<PutBucketArchiveDirectReadParams, ArchiveDirectReadConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketArchiveDirectReadParams::default()),
            body: Some(ArchiveDirectReadConfiguration {
                enabled: self.enabled,
            }),
            ..Default::default()
        })
    }
}

pub trait PutBucketArchiveDirectReadOps {
    /// Enable or disable real-time access of Archive objects.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketarchivedirectread>
    fn put_bucket_archive_direct_read(&self, enabled: bool) -> impl Future<Output = Result<()>>;
}

impl PutBucketArchiveDirectReadOps for Client {
    async fn put_bucket_archive_direct_read(&self, enabled: bool) -> Result<()> {
        self.request(PutBucketArchiveDirectRead { enabled }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutBucketArchiveDirectReadParams::default()).unwrap(),
            "bucketArchiveDirectRead"
        );
    }

    #[test]
    fn body_round_trip() {
        let cfg = ArchiveDirectReadConfiguration { enabled: true };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<Enabled>true</Enabled>"));
        let back: ArchiveDirectReadConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}
