//! ExtendBucketWorm: extend the retention period of a locked WORM policy.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/extendbucketworm>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`ExtendBucketWorm`]: `?wormId=<id>&wormExtend`.
#[derive(Debug, Clone, Serialize)]
pub struct ExtendBucketWormParams {
    #[serde(rename = "wormId")]
    worm_id: String,
    #[serde(rename = "wormExtend")]
    worm_extend: OnlyKeyField,
}

impl ExtendBucketWormParams {
    pub fn new(worm_id: impl Into<String>) -> Self {
        Self {
            worm_id: worm_id.into(),
            worm_extend: OnlyKeyField,
        }
    }
}

/// Request body for [`ExtendBucketWorm`] (`<ExtendWormConfiguration>`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "ExtendWormConfiguration")]
pub struct ExtendWormConfiguration {
    /// The new retention period in days. Valid range: 1 to 25550.
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: u32,
}

impl ExtendWormConfiguration {
    pub fn new(days: u32) -> Self {
        Self {
            retention_period_in_days: days,
        }
    }
}

/// The `ExtendBucketWorm` operation.
pub struct ExtendBucketWorm {
    pub worm_id: String,
    pub config: ExtendWormConfiguration,
}

impl Ops for ExtendBucketWorm {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<ExtendWormConfiguration>;
    type Query = ExtendBucketWormParams;

    fn prepare(self) -> Result<Prepared<ExtendBucketWormParams, ExtendWormConfiguration>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(ExtendBucketWormParams::new(self.worm_id)),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait ExtendBucketWormOps {
    /// Extend the retention period of a locked retention policy.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/extendbucketworm>
    fn extend_bucket_worm(
        &self,
        worm_id: impl Into<String>,
        retention_period_in_days: u32,
    ) -> impl Future<Output = Result<()>>;
}

impl ExtendBucketWormOps for Client {
    async fn extend_bucket_worm(
        &self,
        worm_id: impl Into<String>,
        retention_period_in_days: u32,
    ) -> Result<()> {
        let ops = ExtendBucketWorm {
            worm_id: worm_id.into(),
            config: ExtendWormConfiguration::new(retention_period_in_days),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_both_subresources() {
        // Query string depends on ser::MapSerializer byte-wise lexicographic
        // sort: 'E' (0x45) < 'I' (0x49), so `wormExtend` comes before `wormId`.
        let q = crate::ser::to_string(&ExtendBucketWormParams::new("abc")).unwrap();
        assert_eq!(q, "wormExtend&wormId=abc");
    }

    #[test]
    fn body_serializes_to_expected_xml() {
        let xml = quick_xml::se::to_string(&ExtendWormConfiguration::new(366)).unwrap();
        assert!(xml.contains("<ExtendWormConfiguration>"));
        assert!(xml.contains("<RetentionPeriodInDays>366</RetentionPeriodInDays>"));
    }

    #[test]
    fn prepared_uses_post_with_body() {
        let prepared = ExtendBucketWorm {
            worm_id: "abc".to_string(),
            config: ExtendWormConfiguration::new(10),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::POST);
        assert!(prepared.body.is_some());
    }
}
