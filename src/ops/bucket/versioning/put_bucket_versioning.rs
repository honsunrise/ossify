//! PutBucketVersioning.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketversioning>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketVersioningParams {
    versioning: OnlyKeyField,
}

/// Versioning state of a bucket.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersioningStatus {
    /// OSS stores multiple versions of objects.
    Enabled,
    /// OSS generates the version ID `null` for new objects, and existing
    /// versions are preserved as-is.
    Suspended,
}

impl VersioningStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            VersioningStatus::Enabled => "Enabled",
            VersioningStatus::Suspended => "Suspended",
        }
    }
}

impl AsRef<str> for VersioningStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Body for PutBucketVersioning / GetBucketVersioning.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "VersioningConfiguration", rename_all = "PascalCase")]
pub struct VersioningConfiguration {
    /// `None` if versioning has never been enabled on the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VersioningStatus>,
}

impl VersioningConfiguration {
    pub fn new(status: VersioningStatus) -> Self {
        Self { status: Some(status) }
    }
}

pub struct PutBucketVersioning {
    pub config: VersioningConfiguration,
}

impl Ops for PutBucketVersioning {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<VersioningConfiguration>;
    type Query = PutBucketVersioningParams;

    fn prepare(self) -> Result<Prepared<PutBucketVersioningParams, VersioningConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketVersioningParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketVersioningOps {
    /// Enable or suspend versioning on the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketversioning>
    fn put_bucket_versioning(&self, status: VersioningStatus) -> impl Future<Output = Result<()>>;
}

impl PutBucketVersioningOps for Client {
    async fn put_bucket_versioning(&self, status: VersioningStatus) -> Result<()> {
        self.request(PutBucketVersioning {
            config: VersioningConfiguration::new(status),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutBucketVersioningParams::default()).unwrap(),
            "versioning"
        );
    }

    #[test]
    fn body_round_trip_enabled() {
        let cfg = VersioningConfiguration::new(VersioningStatus::Enabled);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<Status>Enabled</Status>"));
        let back: VersioningConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn body_round_trip_suspended() {
        let cfg = VersioningConfiguration::new(VersioningStatus::Suspended);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<Status>Suspended</Status>"));
    }
}
