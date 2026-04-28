//! DeleteResourcePoolBucketGroupQoSInfo: remove QoS for a bucket-group inside
//! a resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteresourcepoolbucketgroupqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteResourcePoolBucketGroupQoSInfoParams {
    #[serde(rename = "resourcePoolBucketGroupQosInfo")]
    pub(crate) resource_pool_bucket_group_qos_info: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,
    #[serde(rename = "resourcePoolBucketGroup")]
    pub resource_pool_bucket_group: String,
}

impl DeleteResourcePoolBucketGroupQoSInfoParams {
    pub fn new(resource_pool: impl Into<String>, resource_pool_bucket_group: impl Into<String>) -> Self {
        Self {
            resource_pool_bucket_group_qos_info: OnlyKeyField,
            resource_pool: resource_pool.into(),
            resource_pool_bucket_group: resource_pool_bucket_group.into(),
        }
    }
}

pub struct DeleteResourcePoolBucketGroupQoSInfo {
    pub resource_pool: String,
    pub resource_pool_bucket_group: String,
}

impl Ops for DeleteResourcePoolBucketGroupQoSInfo {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteResourcePoolBucketGroupQoSInfoParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<DeleteResourcePoolBucketGroupQoSInfoParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteResourcePoolBucketGroupQoSInfoParams::new(
                self.resource_pool,
                self.resource_pool_bucket_group,
            )),
            ..Default::default()
        })
    }
}

pub trait DeleteResourcePoolBucketGroupQoSInfoOps {
    /// Delete QoS configuration for a bucket-group inside a resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteresourcepoolbucketgroupqosinfo>
    fn delete_resource_pool_bucket_group_qos_info(
        &self,
        resource_pool: impl Into<String>,
        resource_pool_bucket_group: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteResourcePoolBucketGroupQoSInfoOps for Client {
    async fn delete_resource_pool_bucket_group_qos_info(
        &self,
        resource_pool: impl Into<String>,
        resource_pool_bucket_group: impl Into<String>,
    ) -> Result<()> {
        self.request(DeleteResourcePoolBucketGroupQoSInfo {
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
        let q = crate::ser::to_string(&DeleteResourcePoolBucketGroupQoSInfoParams::new(
            "rp-for-ai",
            "test-group",
        ))
        .unwrap();
        assert_eq!(
            q,
            "resourcePool=rp-for-ai&resourcePoolBucketGroup=test-group\
             &resourcePoolBucketGroupQosInfo"
        );
    }

    #[test]
    fn prepare_method() {
        let prepared = DeleteResourcePoolBucketGroupQoSInfo {
            resource_pool: "rp-for-ai".into(),
            resource_pool_bucket_group: "g".into(),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::DELETE);
    }
}
