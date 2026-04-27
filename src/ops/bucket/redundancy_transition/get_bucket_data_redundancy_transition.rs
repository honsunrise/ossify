//! GetBucketDataRedundancyTransition.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketdataredundancytransition>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::RedundancyTransitionStatus;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetBucketDataRedundancyTransitionParams {
    #[serde(rename = "redundancyTransition")]
    redundancy_transition: OnlyKeyField,
    #[serde(rename = "x-oss-redundancy-transition-taskid")]
    pub task_id: String,
}

impl GetBucketDataRedundancyTransitionParams {
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            redundancy_transition: OnlyKeyField,
            task_id: task_id.into(),
        }
    }
}

/// Response body (XML root `<BucketDataRedundancyTransition>`). Includes
/// optional Bucket / CreateTime / StartTime / EndTime / ProcessPercentage /
/// EstimatedRemainingTime fields.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename = "BucketDataRedundancyTransition", rename_all = "PascalCase")]
pub struct GetBucketDataRedundancyTransitionResponse {
    pub bucket: Option<String>,
    pub task_id: String,
    pub status: Option<RedundancyTransitionStatus>,
    pub create_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub process_percentage: Option<u32>,
    pub estimated_remaining_time: Option<u32>,
}

pub struct GetBucketDataRedundancyTransition {
    pub task_id: String,
}

impl Ops for GetBucketDataRedundancyTransition {
    type Response = BodyResponseProcessor<GetBucketDataRedundancyTransitionResponse>;
    type Body = NoneBody;
    type Query = GetBucketDataRedundancyTransitionParams;

    fn prepare(self) -> Result<Prepared<GetBucketDataRedundancyTransitionParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketDataRedundancyTransitionParams::new(self.task_id)),
            ..Default::default()
        })
    }
}

pub trait GetBucketDataRedundancyTransitionOps {
    /// Query the status of a single redundancy type conversion task.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketdataredundancytransition>
    fn get_bucket_data_redundancy_transition(
        &self,
        task_id: impl Into<String>,
    ) -> impl Future<Output = Result<GetBucketDataRedundancyTransitionResponse>>;
}

impl GetBucketDataRedundancyTransitionOps for Client {
    async fn get_bucket_data_redundancy_transition(
        &self,
        task_id: impl Into<String>,
    ) -> Result<GetBucketDataRedundancyTransitionResponse> {
        self.request(GetBucketDataRedundancyTransition {
            task_id: task_id.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&GetBucketDataRedundancyTransitionParams::new("task-1")).unwrap();
        assert_eq!(q, "redundancyTransition&x-oss-redundancy-transition-taskid=task-1");
    }

    #[test]
    fn parse_processing_response() {
        let xml = r#"<BucketDataRedundancyTransition>
  <Bucket>examplebucket</Bucket>
  <TaskId>909c6c818dd041d1a44e0fdc66aa****</TaskId>
  <Status>Processing</Status>
  <CreateTime>2023-11-17T09:14:39.000Z</CreateTime>
  <StartTime>2023-11-17T09:14:39.000Z</StartTime>
  <ProcessPercentage>0</ProcessPercentage>
  <EstimatedRemainingTime>100</EstimatedRemainingTime>
</BucketDataRedundancyTransition>"#;
        let parsed: GetBucketDataRedundancyTransitionResponse = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.status, Some(RedundancyTransitionStatus::Processing));
        assert_eq!(parsed.estimated_remaining_time, Some(100));
    }
}
