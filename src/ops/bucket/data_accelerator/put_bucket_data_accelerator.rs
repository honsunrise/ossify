//! PutBucketDataAccelerator.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketdataaccelerator>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketDataAcceleratorParams {
    #[serde(rename = "dataAccelerator")]
    data_accelerator: OnlyKeyField,
}

/// Cache policy applied to accelerator paths.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CachePolicy {
    #[serde(rename = "sync-warmup")]
    SyncWarmup,
    #[serde(rename = "write-back")]
    WriteBack,
}

impl CachePolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            CachePolicy::SyncWarmup => "sync-warmup",
            CachePolicy::WriteBack => "write-back",
        }
    }
}

impl AsRef<str> for CachePolicy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// `<Path><Name>...</Name><CachePolicy>...</CachePolicy></Path>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Path", rename_all = "PascalCase")]
pub struct AcceleratePath {
    pub name: String,
    pub cache_policy: CachePolicy,
}

/// `<AcceleratePaths>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AcceleratePaths", rename_all = "PascalCase")]
pub struct AcceleratePaths {
    pub default_cache_policy: CachePolicy,
    #[serde(rename = "Path", default)]
    pub paths: Vec<AcceleratePath>,
}

/// Request body: `<DataAcceleratorConfiguration>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "DataAcceleratorConfiguration", rename_all = "PascalCase")]
pub struct DataAcceleratorConfiguration {
    pub available_zone: String,
    pub quota: u64,
    pub accelerate_paths: AcceleratePaths,
}

pub struct PutBucketDataAccelerator {
    pub config: DataAcceleratorConfiguration,
}

impl Ops for PutBucketDataAccelerator {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<DataAcceleratorConfiguration>;
    type Query = PutBucketDataAcceleratorParams;

    fn prepare(self) -> Result<Prepared<PutBucketDataAcceleratorParams, DataAcceleratorConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketDataAcceleratorParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketDataAcceleratorOps {
    /// Create or modify the OSS accelerator configuration for this bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketdataaccelerator>
    fn put_bucket_data_accelerator(
        &self,
        config: DataAcceleratorConfiguration,
    ) -> impl Future<Output = Result<()>>;
}

impl PutBucketDataAcceleratorOps for Client {
    async fn put_bucket_data_accelerator(&self, config: DataAcceleratorConfiguration) -> Result<()> {
        self.request(PutBucketDataAccelerator { config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutBucketDataAcceleratorParams::default()).unwrap(),
            "dataAccelerator"
        );
    }

    #[test]
    fn body_round_trip() {
        let cfg = DataAcceleratorConfiguration {
            available_zone: "cn-wulanchabu-b".to_string(),
            quota: 200,
            accelerate_paths: AcceleratePaths {
                default_cache_policy: CachePolicy::WriteBack,
                paths: vec![AcceleratePath {
                    name: "AccelerationPath".to_string(),
                    cache_policy: CachePolicy::SyncWarmup,
                }],
            },
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<AvailableZone>cn-wulanchabu-b</AvailableZone>"));
        assert!(xml.contains("<Quota>200</Quota>"));
        assert!(xml.contains("<DefaultCachePolicy>write-back</DefaultCachePolicy>"));
        assert!(xml.contains("<Name>AccelerationPath</Name>"));
        assert!(xml.contains("<CachePolicy>sync-warmup</CachePolicy>"));
        let back: DataAcceleratorConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}
