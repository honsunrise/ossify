//! InitiateBucketWorm: create a time-based retention (WORM) policy on a bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/initiatebucketworm>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`InitiateBucketWorm`] (just the `?worm` sub-resource).
#[derive(Debug, Clone, Default, Serialize)]
pub struct InitiateBucketWormParams {
    worm: OnlyKeyField,
}

/// Request body for [`InitiateBucketWorm`] (XML: `<InitiateWormConfiguration>`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "InitiateWormConfiguration")]
pub struct InitiateWormConfiguration {
    /// The retention period for objects in days. Valid range: 1 to 25550.
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: u32,
}

impl InitiateWormConfiguration {
    pub fn new(days: u32) -> Self {
        Self {
            retention_period_in_days: days,
        }
    }
}

/// Response to [`InitiateBucketWorm`]. Parsed from the `x-oss-worm-id`
/// response header.
#[derive(Debug, Clone, Deserialize)]
pub struct InitiateBucketWormResponse {
    /// The ID of the newly created retention policy.
    #[serde(rename = "x-oss-worm-id")]
    pub worm_id: String,
}

/// The `InitiateBucketWorm` operation.
pub struct InitiateBucketWorm {
    pub config: InitiateWormConfiguration,
}

impl Ops for InitiateBucketWorm {
    type Response = HeaderResponseProcessor<InitiateBucketWormResponse>;
    type Body = XMLBody<InitiateWormConfiguration>;
    type Query = InitiateBucketWormParams;

    fn prepare(self) -> Result<Prepared<InitiateBucketWormParams, InitiateWormConfiguration>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(InitiateBucketWormParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait InitiateBucketWormOps {
    /// Create a retention policy on the bucket. Returns the new `WormId`.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/initiatebucketworm>
    fn initiate_bucket_worm(
        &self,
        retention_period_in_days: u32,
    ) -> impl Future<Output = Result<InitiateBucketWormResponse>>;
}

impl InitiateBucketWormOps for Client {
    async fn initiate_bucket_worm(
        &self,
        retention_period_in_days: u32,
    ) -> Result<InitiateBucketWormResponse> {
        let ops = InitiateBucketWorm {
            config: InitiateWormConfiguration::new(retention_period_in_days),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_adds_worm_subresource() {
        let q = crate::ser::to_string(&InitiateBucketWormParams::default()).unwrap();
        assert_eq!(q, "worm");
    }

    #[test]
    fn body_serializes_to_expected_xml() {
        let xml = quick_xml::se::to_string(&InitiateWormConfiguration::new(365)).unwrap();
        assert!(xml.contains("<InitiateWormConfiguration>"));
        assert!(xml.contains("<RetentionPeriodInDays>365</RetentionPeriodInDays>"));
    }

    #[test]
    fn prepared_uses_post() {
        let prepared = InitiateBucketWorm {
            config: InitiateWormConfiguration::new(30),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::POST);
        assert!(prepared.body.is_some());
        assert_eq!(crate::ser::to_string(prepared.query.as_ref().unwrap()).unwrap(), "worm");
    }
}
