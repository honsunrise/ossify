//! GetBucketReplicationLocation.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketreplicationlocation>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::ReplicationLocation;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketReplicationLocationParams {
    #[serde(rename = "replicationLocation")]
    replication_location: OnlyKeyField,
}

pub struct GetBucketReplicationLocation;

impl Ops for GetBucketReplicationLocation {
    type Response = BodyResponseProcessor<ReplicationLocation>;
    type Body = NoneBody;
    type Query = GetBucketReplicationLocationParams;

    fn prepare(self) -> Result<Prepared<GetBucketReplicationLocationParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketReplicationLocationParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketReplicationLocationOps {
    /// List the regions to which the current bucket can replicate.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketreplicationlocation>
    fn get_bucket_replication_location(&self) -> impl Future<Output = Result<ReplicationLocation>>;
}

impl GetBucketReplicationLocationOps for Client {
    async fn get_bucket_replication_location(&self) -> Result<ReplicationLocation> {
        self.request(GetBucketReplicationLocation).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketReplicationLocationParams::default()).unwrap(),
            "replicationLocation"
        );
    }
}
