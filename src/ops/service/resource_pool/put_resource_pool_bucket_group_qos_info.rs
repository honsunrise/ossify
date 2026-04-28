//! PutResourcePoolBucketGroupQoSInfo: set QoS for a bucket-group inside a
//! resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putresourcepoolbucketgroupqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::QoSConfiguration;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct PutResourcePoolBucketGroupQoSInfoParams {
    // Note: Put/Get/Delete use lower-case "Qos" per official documentation;
    // only the List variant uses "QoS" in the URL.
    #[serde(rename = "resourcePoolBucketGroupQosInfo")]
    pub(crate) resource_pool_bucket_group_qos_info: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,
    #[serde(rename = "resourcePoolBucketGroup")]
    pub resource_pool_bucket_group: String,
}

impl PutResourcePoolBucketGroupQoSInfoParams {
    pub fn new(resource_pool: impl Into<String>, resource_pool_bucket_group: impl Into<String>) -> Self {
        Self {
            resource_pool_bucket_group_qos_info: OnlyKeyField,
            resource_pool: resource_pool.into(),
            resource_pool_bucket_group: resource_pool_bucket_group.into(),
        }
    }
}

pub struct PutResourcePoolBucketGroupQoSInfo {
    pub resource_pool: String,
    pub resource_pool_bucket_group: String,
    pub configuration: QoSConfiguration,
}

impl Ops for PutResourcePoolBucketGroupQoSInfo {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<QoSConfiguration>;
    type Query = PutResourcePoolBucketGroupQoSInfoParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<PutResourcePoolBucketGroupQoSInfoParams, QoSConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutResourcePoolBucketGroupQoSInfoParams::new(
                self.resource_pool,
                self.resource_pool_bucket_group,
            )),
            body: Some(self.configuration),
            ..Default::default()
        })
    }
}

pub trait PutResourcePoolBucketGroupQoSInfoOps {
    /// Set QoS configuration for a bucket-group inside a resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putresourcepoolbucketgroupqosinfo>
    fn put_resource_pool_bucket_group_qos_info(
        &self,
        resource_pool: impl Into<String>,
        resource_pool_bucket_group: impl Into<String>,
        configuration: QoSConfiguration,
    ) -> impl Future<Output = Result<()>>;
}

impl PutResourcePoolBucketGroupQoSInfoOps for Client {
    async fn put_resource_pool_bucket_group_qos_info(
        &self,
        resource_pool: impl Into<String>,
        resource_pool_bucket_group: impl Into<String>,
        configuration: QoSConfiguration,
    ) -> Result<()> {
        self.request(PutResourcePoolBucketGroupQoSInfo {
            resource_pool: resource_pool.into(),
            resource_pool_bucket_group: resource_pool_bucket_group.into(),
            configuration,
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
            crate::ser::to_string(&PutResourcePoolBucketGroupQoSInfoParams::new("rp-for-ai", "test-group"))
                .unwrap();
        assert_eq!(
            q,
            "resourcePool=rp-for-ai&resourcePoolBucketGroup=test-group\
             &resourcePoolBucketGroupQosInfo"
        );
    }

    #[test]
    fn body_round_trip() {
        let cfg = QoSConfiguration::bandwidth(10, -1, -1, 10, -1, -1);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        let back: QoSConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}
