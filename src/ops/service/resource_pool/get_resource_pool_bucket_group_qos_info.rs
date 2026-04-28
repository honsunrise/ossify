//! GetResourcePoolBucketGroupQoSInfo: fetch QoS for a bucket-group inside a
//! resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getresourcepoolbucketgroupqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::ResourcePoolBucketGroupQoSInfo;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetResourcePoolBucketGroupQoSInfoParams {
    #[serde(rename = "resourcePoolBucketGroupQosInfo")]
    pub(crate) resource_pool_bucket_group_qos_info: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,
    #[serde(rename = "resourcePoolBucketGroup")]
    pub resource_pool_bucket_group: String,
}

impl GetResourcePoolBucketGroupQoSInfoParams {
    pub fn new(resource_pool: impl Into<String>, resource_pool_bucket_group: impl Into<String>) -> Self {
        Self {
            resource_pool_bucket_group_qos_info: OnlyKeyField,
            resource_pool: resource_pool.into(),
            resource_pool_bucket_group: resource_pool_bucket_group.into(),
        }
    }
}

pub struct GetResourcePoolBucketGroupQoSInfo {
    pub resource_pool: String,
    pub resource_pool_bucket_group: String,
}

impl Ops for GetResourcePoolBucketGroupQoSInfo {
    type Response = BodyResponseProcessor<ResourcePoolBucketGroupQoSInfo>;
    type Body = NoneBody;
    type Query = GetResourcePoolBucketGroupQoSInfoParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<GetResourcePoolBucketGroupQoSInfoParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetResourcePoolBucketGroupQoSInfoParams::new(
                self.resource_pool,
                self.resource_pool_bucket_group,
            )),
            ..Default::default()
        })
    }
}

pub trait GetResourcePoolBucketGroupQoSInfoOps {
    /// Fetch QoS for a bucket-group inside a resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getresourcepoolbucketgroupqosinfo>
    fn get_resource_pool_bucket_group_qos_info(
        &self,
        resource_pool: impl Into<String>,
        resource_pool_bucket_group: impl Into<String>,
    ) -> impl Future<Output = Result<ResourcePoolBucketGroupQoSInfo>>;
}

impl GetResourcePoolBucketGroupQoSInfoOps for Client {
    async fn get_resource_pool_bucket_group_qos_info(
        &self,
        resource_pool: impl Into<String>,
        resource_pool_bucket_group: impl Into<String>,
    ) -> Result<ResourcePoolBucketGroupQoSInfo> {
        self.request(GetResourcePoolBucketGroupQoSInfo {
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
            crate::ser::to_string(&GetResourcePoolBucketGroupQoSInfoParams::new("rp-for-ai", "test-group"))
                .unwrap();
        assert_eq!(
            q,
            "resourcePool=rp-for-ai&resourcePoolBucketGroup=test-group\
             &resourcePoolBucketGroupQosInfo"
        );
    }

    #[test]
    fn parse_response_with_bucket_group_tag() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ResourcePoolBucketGroupQoSInfo>
  <BucketGroup>test-group</BucketGroup>
  <QoSConfiguration>
    <TotalUploadBandwidth>10</TotalUploadBandwidth>
    <IntranetUploadBandwidth>-1</IntranetUploadBandwidth>
    <ExtranetUploadBandwidth>-1</ExtranetUploadBandwidth>
    <TotalDownloadBandwidth>10</TotalDownloadBandwidth>
    <IntranetDownloadBandwidth>-1</IntranetDownloadBandwidth>
    <ExtranetDownloadBandwidth>-1</ExtranetDownloadBandwidth>
  </QoSConfiguration>
</ResourcePoolBucketGroupQoSInfo>"#;
        let parsed: ResourcePoolBucketGroupQoSInfo = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.bucket_group, "test-group");
    }
}
