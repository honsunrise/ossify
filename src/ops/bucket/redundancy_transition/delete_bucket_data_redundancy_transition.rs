//! DeleteBucketDataRedundancyTransition.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketdataredundancytransition>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteBucketDataRedundancyTransitionParams {
    #[serde(rename = "redundancyTransition")]
    redundancy_transition: OnlyKeyField,
    #[serde(rename = "x-oss-redundancy-transition-taskid")]
    pub task_id: String,
}

impl DeleteBucketDataRedundancyTransitionParams {
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            redundancy_transition: OnlyKeyField,
            task_id: task_id.into(),
        }
    }
}

pub struct DeleteBucketDataRedundancyTransition {
    pub task_id: String,
}

impl Ops for DeleteBucketDataRedundancyTransition {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketDataRedundancyTransitionParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketDataRedundancyTransitionParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketDataRedundancyTransitionParams::new(self.task_id)),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketDataRedundancyTransitionOps {
    /// Delete a redundancy type conversion task. Tasks in Processing state
    /// cannot be deleted.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketdataredundancytransition>
    fn delete_bucket_data_redundancy_transition(
        &self,
        task_id: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketDataRedundancyTransitionOps for Client {
    async fn delete_bucket_data_redundancy_transition(&self, task_id: impl Into<String>) -> Result<()> {
        self.request(DeleteBucketDataRedundancyTransition {
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
        let q = crate::ser::to_string(&DeleteBucketDataRedundancyTransitionParams::new("task-1")).unwrap();
        assert_eq!(q, "redundancyTransition&x-oss-redundancy-transition-taskid=task-1");
    }
}
