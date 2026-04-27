//! ListBucketDataRedundancyTransition.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbucketdataredundancytransition>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::RedundancyTransitionStatus;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListBucketDataRedundancyTransitionParams {
    #[serde(rename = "redundancyTransition")]
    redundancy_transition: OnlyKeyField,
}

/// One entry in the list response.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketDataRedundancyTransitionEntry {
    pub bucket: Option<String>,
    pub task_id: String,
    pub status: Option<RedundancyTransitionStatus>,
    pub create_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub process_percentage: Option<u32>,
    pub estimated_remaining_time: Option<u32>,
}

/// Response body (XML root `<ListBucketDataRedundancyTransition>`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename = "ListBucketDataRedundancyTransition", rename_all = "PascalCase")]
pub struct ListBucketDataRedundancyTransitionResult {
    #[serde(rename = "BucketDataRedundancyTransition", default)]
    pub transitions: Vec<BucketDataRedundancyTransitionEntry>,
}

pub struct ListBucketDataRedundancyTransition;

impl Ops for ListBucketDataRedundancyTransition {
    type Response = BodyResponseProcessor<ListBucketDataRedundancyTransitionResult>;
    type Body = NoneBody;
    type Query = ListBucketDataRedundancyTransitionParams;

    fn prepare(self) -> Result<Prepared<ListBucketDataRedundancyTransitionParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(ListBucketDataRedundancyTransitionParams::default()),
            ..Default::default()
        })
    }
}

pub trait ListBucketDataRedundancyTransitionOps {
    /// List all redundancy-type-conversion tasks on the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbucketdataredundancytransition>
    fn list_bucket_data_redundancy_transition(
        &self,
    ) -> impl Future<Output = Result<ListBucketDataRedundancyTransitionResult>>;
}

impl ListBucketDataRedundancyTransitionOps for Client {
    async fn list_bucket_data_redundancy_transition(
        &self,
    ) -> Result<ListBucketDataRedundancyTransitionResult> {
        self.request(ListBucketDataRedundancyTransition).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&ListBucketDataRedundancyTransitionParams::default()).unwrap(),
            "redundancyTransition"
        );
    }

    #[test]
    fn parse_list_response() {
        let xml = r#"<ListBucketDataRedundancyTransition>
  <BucketDataRedundancyTransition>
    <Bucket>examplebucket</Bucket>
    <TaskId>task-1</TaskId>
    <Status>Queueing</Status>
    <CreateTime>2023-11-17T08:40:17.000Z</CreateTime>
  </BucketDataRedundancyTransition>
  <BucketDataRedundancyTransition>
    <Bucket>examplebucket</Bucket>
    <TaskId>task-2</TaskId>
    <Status>Finished</Status>
  </BucketDataRedundancyTransition>
</ListBucketDataRedundancyTransition>"#;
        let parsed: ListBucketDataRedundancyTransitionResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.transitions.len(), 2);
        assert_eq!(parsed.transitions[1].task_id, "task-2");
    }
}
