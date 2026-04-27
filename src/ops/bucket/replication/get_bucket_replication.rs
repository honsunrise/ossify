//! GetBucketReplication.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketreplication>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::ReplicationConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketReplicationParams {
    replication: OnlyKeyField,
}

pub struct GetBucketReplication;

impl Ops for GetBucketReplication {
    type Response = BodyResponseProcessor<ReplicationConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketReplicationParams;

    fn prepare(self) -> Result<Prepared<GetBucketReplicationParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketReplicationParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketReplicationOps {
    /// Retrieve the replication rules configured on the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketreplication>
    fn get_bucket_replication(&self) -> impl Future<Output = Result<ReplicationConfiguration>>;
}

impl GetBucketReplicationOps for Client {
    async fn get_bucket_replication(&self) -> Result<ReplicationConfiguration> {
        self.request(GetBucketReplication).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketReplicationParams::default()).unwrap(),
            "replication"
        );
    }

    #[test]
    fn prepared_uses_get() {
        let prepared = GetBucketReplication.prepare().unwrap();
        assert_eq!(prepared.method, Method::GET);
    }
}
