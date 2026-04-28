//! PutBucketResourcePoolBucketGroup: assign (or clear) the bucket-group that
//! this bucket belongs to inside a resource pool.
//!
//! Pass an empty `resource_pool_bucket_group` to remove the bucket from its
//! current group inside the given resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketresourcepoolbucketgroup>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct PutBucketResourcePoolBucketGroupParams {
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,
    #[serde(rename = "resourcePoolBucketGroup")]
    pub resource_pool_bucket_group: String,
}

impl PutBucketResourcePoolBucketGroupParams {
    pub fn new(resource_pool: impl Into<String>, resource_pool_bucket_group: impl Into<String>) -> Self {
        Self {
            resource_pool: resource_pool.into(),
            resource_pool_bucket_group: resource_pool_bucket_group.into(),
        }
    }
}

pub struct PutBucketResourcePoolBucketGroup {
    pub resource_pool: String,
    pub resource_pool_bucket_group: String,
}

impl Ops for PutBucketResourcePoolBucketGroup {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = PutBucketResourcePoolBucketGroupParams;

    fn prepare(self) -> Result<Prepared<PutBucketResourcePoolBucketGroupParams>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketResourcePoolBucketGroupParams::new(
                self.resource_pool,
                self.resource_pool_bucket_group,
            )),
            ..Default::default()
        })
    }
}

pub trait PutBucketResourcePoolBucketGroupOps {
    /// Place the bucket into (or out of) a bucket-group inside a resource pool.
    ///
    /// Pass an empty group name to remove the bucket from its current group.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketresourcepoolbucketgroup>
    fn put_bucket_resource_pool_bucket_group(
        &self,
        resource_pool: impl Into<String>,
        resource_pool_bucket_group: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutBucketResourcePoolBucketGroupOps for Client {
    async fn put_bucket_resource_pool_bucket_group(
        &self,
        resource_pool: impl Into<String>,
        resource_pool_bucket_group: impl Into<String>,
    ) -> Result<()> {
        self.request(PutBucketResourcePoolBucketGroup {
            resource_pool: resource_pool.into(),
            resource_pool_bucket_group: resource_pool_bucket_group.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q =
            crate::ser::to_string(&PutBucketResourcePoolBucketGroupParams::new("rp-for-ai", "test-group"))
                .unwrap();
        assert_eq!(q, "resourcePool=rp-for-ai&resourcePoolBucketGroup=test-group");
    }

    #[test]
    fn prepare_method() {
        let prepared = PutBucketResourcePoolBucketGroup {
            resource_pool: "rp-for-ai".into(),
            resource_pool_bucket_group: "g".into(),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::PUT);
    }
}
