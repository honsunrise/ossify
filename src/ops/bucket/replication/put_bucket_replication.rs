//! PutBucketReplication.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketreplication>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::ReplicationConfiguration;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct PutBucketReplicationParams {
    replication: OnlyKeyField,
    comp: String,
}

impl Default for PutBucketReplicationParams {
    fn default() -> Self {
        Self {
            replication: OnlyKeyField,
            comp: "add".to_string(),
        }
    }
}

/// The `PutBucketReplication` operation.
pub struct PutBucketReplication {
    pub config: ReplicationConfiguration,
}

impl Ops for PutBucketReplication {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<ReplicationConfiguration>;
    type Query = PutBucketReplicationParams;

    fn prepare(self) -> Result<Prepared<PutBucketReplicationParams, ReplicationConfiguration>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(PutBucketReplicationParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketReplicationOps {
    /// Create a data replication rule for the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketreplication>
    fn put_bucket_replication(&self, config: ReplicationConfiguration) -> impl Future<Output = Result<()>>;
}

impl PutBucketReplicationOps for Client {
    async fn put_bucket_replication(&self, config: ReplicationConfiguration) -> Result<()> {
        self.request(PutBucketReplication { config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::common::{
        HistoricalObjectReplication,
        ReplicationAction,
        ReplicationDestination,
        ReplicationRule,
        TransferType,
    };

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&PutBucketReplicationParams::default()).unwrap();
        assert_eq!(q, "comp=add&replication");
    }

    #[test]
    fn prepared_has_post_and_xml_body() {
        let cfg = ReplicationConfiguration::with_rules(vec![ReplicationRule {
            action: Some(ReplicationAction::All),
            destination: Some(ReplicationDestination {
                bucket: "b".to_string(),
                location: "oss-cn-beijing".to_string(),
                transfer_type: Some(TransferType::Internal),
            }),
            historical_object_replication: Some(HistoricalObjectReplication::Disabled),
            sync_role: Some("r".to_string()),
            ..Default::default()
        }]);
        let prepared = PutBucketReplication { config: cfg }.prepare().unwrap();
        assert_eq!(prepared.method, Method::POST);
        assert!(prepared.body.is_some());
    }
}
