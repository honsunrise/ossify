//! DeleteBucketReplication.
//!
//! Official document: <https://help.aliyun.com/zh/oss/developer-reference/deletebucketreplication>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteBucketReplicationParams {
    replication: OnlyKeyField,
    comp: String,
}

impl Default for DeleteBucketReplicationParams {
    fn default() -> Self {
        Self {
            replication: OnlyKeyField,
            comp: "delete".to_string(),
        }
    }
}

/// Request body: `<ReplicationRules><ID>rule id</ID></ReplicationRules>`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "ReplicationRules", rename_all = "PascalCase")]
pub struct DeleteBucketReplicationBody {
    #[serde(rename = "ID")]
    pub id: String,
}

impl DeleteBucketReplicationBody {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

pub struct DeleteBucketReplication {
    pub rule_id: String,
}

impl Ops for DeleteBucketReplication {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<DeleteBucketReplicationBody>;
    type Query = DeleteBucketReplicationParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketReplicationParams, DeleteBucketReplicationBody>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(DeleteBucketReplicationParams::default()),
            body: Some(DeleteBucketReplicationBody::new(self.rule_id)),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketReplicationOps {
    /// Stop and delete a replication rule.
    ///
    /// Official document: <https://help.aliyun.com/zh/oss/developer-reference/deletebucketreplication>
    fn delete_bucket_replication(&self, rule_id: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketReplicationOps for Client {
    async fn delete_bucket_replication(&self, rule_id: impl Into<String>) -> Result<()> {
        self.request(DeleteBucketReplication {
            rule_id: rule_id.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&DeleteBucketReplicationParams::default()).unwrap();
        assert_eq!(q, "comp=delete&replication");
    }

    #[test]
    fn body_serializes() {
        let xml = quick_xml::se::to_string(&DeleteBucketReplicationBody::new("rule-1")).unwrap();
        assert!(xml.contains("<ReplicationRules>"));
        assert!(xml.contains("<ID>rule-1</ID>"));
    }

    #[test]
    fn prepared_uses_post() {
        let prepared = DeleteBucketReplication {
            rule_id: "r".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::POST);
        assert!(prepared.body.is_some());
    }
}
