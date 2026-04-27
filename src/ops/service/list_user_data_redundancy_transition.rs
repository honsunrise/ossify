//! ListUserDataRedundancyTransition: list all data redundancy transition
//! tasks of the current account.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listuserdataredundancytransition>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::RedundancyTransitionStatus;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`ListUserDataRedundancyTransition`].
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListUserDataRedundancyTransitionParams {
    // The sub-resource key for this API. All sibling APIs
    // (CreateBucketDataRedundancyTransition, ListBucketDataRedundancyTransition,
    // GetBucketDataRedundancyTransition, etc.) and the Python SDK v2 use
    // `redundancyTransition`, matching the official "Request syntax" section.
    // The `dataRedundancyTransition` name that appears in a Sample request in
    // the same doc is believed to be a documentation typo.
    #[serde(rename = "redundancyTransition")]
    redundancy_transition: OnlyKeyField,

    /// Pagination token returned by a previous request in
    /// `NextContinuationToken`.
    pub continuation_token: Option<String>,

    /// Maximum number of tasks to return. Valid range: 1-100.
    pub max_keys: Option<u32>,
}

impl ListUserDataRedundancyTransitionParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn continuation_token(mut self, token: impl Into<String>) -> Self {
        self.continuation_token = Some(token.into());
        self
    }

    pub fn max_keys(mut self, max_keys: u32) -> Self {
        self.max_keys = Some(max_keys);
        self
    }
}

/// One task in the response of [`ListUserDataRedundancyTransition`].
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BucketDataRedundancyTransition {
    pub bucket: String,
    pub task_id: String,
    pub status: Option<RedundancyTransitionStatus>,
    pub create_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    /// Progress in percent; set only when status is `Processing` or
    /// `Finished`.
    pub process_percentage: Option<u32>,
    /// Estimated remaining time in hours; set only when status is `Processing`
    /// or `Finished`.
    pub estimated_remaining_time: Option<u32>,
}

/// Response body for [`ListUserDataRedundancyTransition`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUserDataRedundancyTransitionResult {
    /// Whether more results are available.
    #[serde(default)]
    pub is_truncated: bool,
    /// Pagination token to pass back as `continuation-token` in the next
    /// request. Empty when there are no more results.
    pub next_continuation_token: Option<String>,
    /// Individual transition tasks. Each `<BucketDataRedundancyTransition>`
    /// child is one entry.
    #[serde(default, rename = "BucketDataRedundancyTransition")]
    pub transitions: Vec<BucketDataRedundancyTransition>,
}

/// The `ListUserDataRedundancyTransition` operation.
pub struct ListUserDataRedundancyTransition {
    pub params: ListUserDataRedundancyTransitionParams,
}

impl Ops for ListUserDataRedundancyTransition {
    type Response = BodyResponseProcessor<ListUserDataRedundancyTransitionResult>;
    type Body = NoneBody;
    type Query = ListUserDataRedundancyTransitionParams;

    // This is a service-level (account-level) operation: the request goes to
    // the regional service endpoint, not a bucket-scoped host.
    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<ListUserDataRedundancyTransitionParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for the `ListUserDataRedundancyTransition` operation.
pub trait ListUserDataRedundancyTransitionOps {
    /// Lists all data redundancy transition tasks of the current account.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listuserdataredundancytransition>
    fn list_user_data_redundancy_transition(
        &self,
        params: Option<ListUserDataRedundancyTransitionParams>,
    ) -> impl Future<Output = Result<ListUserDataRedundancyTransitionResult>>;
}

impl ListUserDataRedundancyTransitionOps for Client {
    async fn list_user_data_redundancy_transition(
        &self,
        params: Option<ListUserDataRedundancyTransitionParams>,
    ) -> Result<ListUserDataRedundancyTransitionResult> {
        let ops = ListUserDataRedundancyTransition {
            params: params.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_defaults() {
        let q = crate::ser::to_string(&ListUserDataRedundancyTransitionParams::new()).unwrap();
        assert_eq!(q, "redundancyTransition");
    }

    #[test]
    fn params_serialize_with_pagination() {
        let q = crate::ser::to_string(
            &ListUserDataRedundancyTransitionParams::new()
                .continuation_token("abc")
                .max_keys(10),
        )
        .unwrap();
        // Params are sorted alphabetically by the query serializer.
        assert_eq!(q, "continuation-token=abc&max-keys=10&redundancyTransition");
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<ListUserDataRedundancyTransition as Ops>::USE_BUCKET);
    }

    #[test]
    fn parse_response_with_mixed_statuses() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketDataRedundancyTransition>
  <IsTruncated>false</IsTruncated>
  <NextContinuationToken></NextContinuationToken>
  <BucketDataRedundancyTransition>
    <Bucket>examplebucket1</Bucket>
    <TaskId>4be5beb0f74f490186311b268bf6****</TaskId>
    <Status>Queueing</Status>
    <CreateTime>2023-11-17T08:40:17.000Z</CreateTime>
  </BucketDataRedundancyTransition>
  <BucketDataRedundancyTransition>
    <Bucket>examplebucket2</Bucket>
    <TaskId>4be5beb0f74f490186311b268bf6j****</TaskId>
    <Status>Processing</Status>
    <CreateTime>2023-11-17T08:40:17.000Z</CreateTime>
    <StartTime>2023-11-17T10:40:17.000Z</StartTime>
    <ProcessPercentage>50</ProcessPercentage>
    <EstimatedRemainingTime>16</EstimatedRemainingTime>
  </BucketDataRedundancyTransition>
  <BucketDataRedundancyTransition>
    <Bucket>examplebucket3</Bucket>
    <TaskId>4be5beb0er4f490186311b268bf6j****</TaskId>
    <Status>Finished</Status>
    <CreateTime>2023-11-17T08:40:17.000Z</CreateTime>
    <StartTime>2023-11-17T11:40:17.000Z</StartTime>
    <ProcessPercentage>100</ProcessPercentage>
    <EstimatedRemainingTime>0</EstimatedRemainingTime>
    <EndTime>2023-11-18T09:40:17.000Z</EndTime>
  </BucketDataRedundancyTransition>
</ListBucketDataRedundancyTransition>"#;
        let parsed: ListUserDataRedundancyTransitionResult = quick_xml::de::from_str(xml).unwrap();
        assert!(!parsed.is_truncated);
        assert_eq!(parsed.transitions.len(), 3);
        assert_eq!(parsed.transitions[0].status, Some(RedundancyTransitionStatus::Queueing));
        assert_eq!(parsed.transitions[1].status, Some(RedundancyTransitionStatus::Processing));
        assert_eq!(parsed.transitions[1].process_percentage, Some(50));
        assert_eq!(parsed.transitions[2].status, Some(RedundancyTransitionStatus::Finished));
        assert_eq!(parsed.transitions[2].end_time.as_deref(), Some("2023-11-18T09:40:17.000Z"));
    }

    #[test]
    fn parse_response_with_single_transition() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketDataRedundancyTransition>
  <IsTruncated>true</IsTruncated>
  <NextContinuationToken>next-page</NextContinuationToken>
  <BucketDataRedundancyTransition>
    <Bucket>onlybucket</Bucket>
    <TaskId>only-task</TaskId>
    <Status>Queueing</Status>
  </BucketDataRedundancyTransition>
</ListBucketDataRedundancyTransition>"#;
        let parsed: ListUserDataRedundancyTransitionResult = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.is_truncated);
        assert_eq!(parsed.next_continuation_token.as_deref(), Some("next-page"));
        assert_eq!(parsed.transitions.len(), 1);
    }

    #[test]
    fn parse_response_with_zero_transitions() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketDataRedundancyTransition>
  <IsTruncated>false</IsTruncated>
</ListBucketDataRedundancyTransition>"#;
        let parsed: ListUserDataRedundancyTransitionResult = quick_xml::de::from_str(xml).unwrap();
        assert!(!parsed.is_truncated);
        assert_eq!(parsed.transitions.len(), 0);
    }
}
