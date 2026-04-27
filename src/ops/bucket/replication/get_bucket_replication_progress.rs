//! GetBucketReplicationProgress.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketreplicationprogress>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::ReplicationRule;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetBucketReplicationProgressParams {
    #[serde(rename = "replicationProgress")]
    replication_progress: OnlyKeyField,
    pub rule_id: String,
}

impl GetBucketReplicationProgressParams {
    pub fn new(rule_id: impl Into<String>) -> Self {
        Self {
            replication_progress: OnlyKeyField,
            rule_id: rule_id.into(),
        }
    }
}

/// Response body (XML root `<ReplicationProgress>`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename = "ReplicationProgress", rename_all = "PascalCase")]
pub struct ReplicationProgress {
    #[serde(rename = "Rule", default)]
    pub rules: Vec<ReplicationRule>,
}

pub struct GetBucketReplicationProgress {
    pub params: GetBucketReplicationProgressParams,
}

impl Ops for GetBucketReplicationProgress {
    type Response = BodyResponseProcessor<ReplicationProgress>;
    type Body = NoneBody;
    type Query = GetBucketReplicationProgressParams;

    fn prepare(self) -> Result<Prepared<GetBucketReplicationProgressParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait GetBucketReplicationProgressOps {
    /// Query the replication progress for a specific rule.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketreplicationprogress>
    fn get_bucket_replication_progress(
        &self,
        rule_id: impl Into<String>,
    ) -> impl Future<Output = Result<ReplicationProgress>>;
}

impl GetBucketReplicationProgressOps for Client {
    async fn get_bucket_replication_progress(
        &self,
        rule_id: impl Into<String>,
    ) -> Result<ReplicationProgress> {
        self.request(GetBucketReplicationProgress {
            params: GetBucketReplicationProgressParams::new(rule_id),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&GetBucketReplicationProgressParams::new("r-1")).unwrap();
        assert_eq!(q, "replicationProgress&rule-id=r-1");
    }

    #[test]
    fn parse_progress_response() {
        let xml = r#"<ReplicationProgress>
  <Rule>
    <ID>test_replication_1</ID>
    <Action>PUT</Action>
    <Destination>
      <Bucket>target</Bucket>
      <Location>oss-cn-beijing</Location>
    </Destination>
    <Status>doing</Status>
    <Progress>
      <HistoricalObject>0.85</HistoricalObject>
      <NewObject>2015-09-24T15:28:14.000Z</NewObject>
    </Progress>
  </Rule>
</ReplicationProgress>"#;
        let parsed: ReplicationProgress = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.rules.len(), 1);
        assert_eq!(parsed.rules[0].id.as_deref(), Some("test_replication_1"));
        let p = parsed.rules[0].progress.as_ref().unwrap();
        assert_eq!(p.historical_object.as_deref(), Some("0.85"));
    }
}
