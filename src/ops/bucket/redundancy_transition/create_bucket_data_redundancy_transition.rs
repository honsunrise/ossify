//! CreateBucketDataRedundancyTransition.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createbucketdataredundancytransition>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::ZeroBody;
use crate::error::Result;
use crate::ops::common::DataRedundancyType;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct CreateBucketDataRedundancyTransitionParams {
    #[serde(rename = "redundancyTransition")]
    redundancy_transition: OnlyKeyField,
    #[serde(rename = "x-oss-target-redundancy-type")]
    pub target_redundancy_type: DataRedundancyType,
}

impl CreateBucketDataRedundancyTransitionParams {
    pub fn new(target: DataRedundancyType) -> Self {
        Self {
            redundancy_transition: OnlyKeyField,
            target_redundancy_type: target,
        }
    }
}

/// Response body (XML root `<BucketDataRedundancyTransition>`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename = "BucketDataRedundancyTransition", rename_all = "PascalCase")]
pub struct CreateBucketDataRedundancyTransitionResponse {
    pub task_id: String,
}

pub struct CreateBucketDataRedundancyTransition {
    pub target_redundancy_type: DataRedundancyType,
}

impl Ops for CreateBucketDataRedundancyTransition {
    type Response = BodyResponseProcessor<CreateBucketDataRedundancyTransitionResponse>;
    type Body = ZeroBody;
    type Query = CreateBucketDataRedundancyTransitionParams;

    fn prepare(self) -> Result<Prepared<CreateBucketDataRedundancyTransitionParams>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(CreateBucketDataRedundancyTransitionParams::new(self.target_redundancy_type)),
            body: Some(()),
            ..Default::default()
        })
    }
}

pub trait CreateBucketDataRedundancyTransitionOps {
    /// Start a redundancy type conversion task on the bucket.
    ///
    /// Only `LRS -> ZRS` is supported.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createbucketdataredundancytransition>
    fn create_bucket_data_redundancy_transition(
        &self,
        target: DataRedundancyType,
    ) -> impl Future<Output = Result<CreateBucketDataRedundancyTransitionResponse>>;
}

impl CreateBucketDataRedundancyTransitionOps for Client {
    async fn create_bucket_data_redundancy_transition(
        &self,
        target: DataRedundancyType,
    ) -> Result<CreateBucketDataRedundancyTransitionResponse> {
        self.request(CreateBucketDataRedundancyTransition {
            target_redundancy_type: target,
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&CreateBucketDataRedundancyTransitionParams::new(
            DataRedundancyType::ZoneRedundantStorage,
        ))
        .unwrap();
        assert_eq!(q, "redundancyTransition&x-oss-target-redundancy-type=ZRS");
    }

    #[test]
    fn parse_response() {
        let xml =
            r#"<BucketDataRedundancyTransition><TaskId>task-1</TaskId></BucketDataRedundancyTransition>"#;
        let parsed: CreateBucketDataRedundancyTransitionResponse = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.task_id, "task-1");
    }
}
