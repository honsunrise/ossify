//! GetBucketWorm: query the current retention (WORM) policy of a bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketworm>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketWormParams {
    worm: OnlyKeyField,
}

/// State of a WORM retention policy.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum WormState {
    /// The policy was just created; not locked yet. Valid for 24 hours.
    InProgress,
    /// The policy has been locked via `CompleteBucketWorm`.
    Locked,
}

impl WormState {
    pub fn as_str(&self) -> &'static str {
        match self {
            WormState::InProgress => "InProgress",
            WormState::Locked => "Locked",
        }
    }
}

impl AsRef<str> for WormState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Response body for [`GetBucketWorm`] (XML root `<WormConfiguration>`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WormConfiguration {
    /// Unique ID of the retention policy, assigned by OSS.
    pub worm_id: String,
    /// State of the policy.
    pub state: WormState,
    /// Current retention period in days.
    pub retention_period_in_days: u32,
    /// ISO8601 creation time of the policy.
    pub creation_date: String,
    /// ISO8601 expiration time of the policy, if set. Present only for locked
    /// policies that have been in effect long enough for OSS to compute it.
    pub expiration_date: Option<String>,
}

/// The `GetBucketWorm` operation.
pub struct GetBucketWorm;

impl Ops for GetBucketWorm {
    type Response = BodyResponseProcessor<WormConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketWormParams;

    fn prepare(self) -> Result<Prepared<GetBucketWormParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketWormParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketWormOps {
    /// Query the retention (WORM) policy configured on the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketworm>
    fn get_bucket_worm(&self) -> impl Future<Output = Result<WormConfiguration>>;
}

impl GetBucketWormOps for Client {
    async fn get_bucket_worm(&self) -> Result<WormConfiguration> {
        self.request(GetBucketWorm).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_adds_worm_subresource() {
        let q = crate::ser::to_string(&GetBucketWormParams::default()).unwrap();
        assert_eq!(q, "worm");
    }

    #[test]
    fn prepared_uses_get() {
        let prepared = GetBucketWorm.prepare().unwrap();
        assert_eq!(prepared.method, Method::GET);
        assert!(prepared.body.is_none());
    }

    #[test]
    fn parse_locked_policy() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<WormConfiguration>
  <WormId>1666E2CFB2B3418****</WormId>
  <State>Locked</State>
  <RetentionPeriodInDays>1</RetentionPeriodInDays>
  <CreationDate>2020-10-15T15:50:32</CreationDate>
</WormConfiguration>"#;
        let parsed: WormConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.worm_id, "1666E2CFB2B3418****");
        assert_eq!(parsed.state, WormState::Locked);
        assert_eq!(parsed.retention_period_in_days, 1);
    }

    #[test]
    fn parse_in_progress_policy() {
        let xml = r#"<WormConfiguration>
  <WormId>abc</WormId>
  <State>InProgress</State>
  <RetentionPeriodInDays>365</RetentionPeriodInDays>
  <CreationDate>2024-01-01T00:00:00</CreationDate>
</WormConfiguration>"#;
        let parsed: WormConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.state, WormState::InProgress);
        assert_eq!(parsed.retention_period_in_days, 365);
        assert!(parsed.expiration_date.is_none());
    }

    #[test]
    fn parse_locked_policy_with_expiration_date() {
        // ExpirationDate is not mentioned in the English docs but Python SDK
        // v2 exposes it and OSS does return it in practice.
        let xml = r#"<WormConfiguration>
  <WormId>abc</WormId>
  <State>Locked</State>
  <RetentionPeriodInDays>30</RetentionPeriodInDays>
  <CreationDate>2024-01-01T00:00:00</CreationDate>
  <ExpirationDate>2024-01-31T00:00:00</ExpirationDate>
</WormConfiguration>"#;
        let parsed: WormConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.expiration_date.as_deref(), Some("2024-01-31T00:00:00"));
    }
}
