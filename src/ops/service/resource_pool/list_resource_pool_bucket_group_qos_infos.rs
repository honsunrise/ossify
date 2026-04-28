//! ListResourcePoolBucketGroupQoSInfos: list QoS configurations for all
//! bucket-groups in a resource pool.
//!
//! Note: per the official documentation, the URL query key here uses
//! "QoS" (capital `S`), unlike the singular Put/Get/Delete variants which
//! use lowercase "Qos".
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepoolbucketgroupqosinfos>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::ResourcePoolBucketGroupQoSInfo;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListResourcePoolBucketGroupQoSInfosParams {
    // Capital "QoS" per documentation (the List variant differs from the
    // singular URL query key).
    #[serde(rename = "resourcePoolBucketGroupQoSInfo")]
    pub(crate) resource_pool_bucket_group_qos_info: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,

    pub continuation_token: Option<String>,
    pub max_keys: Option<u32>,
}

impl ListResourcePoolBucketGroupQoSInfosParams {
    pub fn new(resource_pool: impl Into<String>) -> Self {
        Self {
            resource_pool_bucket_group_qos_info: OnlyKeyField,
            resource_pool: resource_pool.into(),
            continuation_token: None,
            max_keys: None,
        }
    }

    pub fn continuation_token(mut self, token: impl Into<String>) -> Self {
        self.continuation_token = Some(token.into());
        self
    }

    pub fn max_keys(mut self, max_keys: u32) -> Self {
        self.max_keys = Some(max_keys);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListResourcePoolBucketGroupQoSInfosResult {
    pub resource_pool: Option<String>,
    pub continuation_token: Option<String>,
    pub next_continuation_token: Option<String>,
    #[serde(default)]
    pub is_truncated: bool,
    #[serde(default, rename = "ResourcePoolBucketGroupQoSInfo")]
    pub bucket_group_qos_infos: Vec<ResourcePoolBucketGroupQoSInfo>,
}

pub struct ListResourcePoolBucketGroupQoSInfos {
    pub params: ListResourcePoolBucketGroupQoSInfosParams,
}

impl Ops for ListResourcePoolBucketGroupQoSInfos {
    type Response = BodyResponseProcessor<ListResourcePoolBucketGroupQoSInfosResult>;
    type Body = NoneBody;
    type Query = ListResourcePoolBucketGroupQoSInfosParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<ListResourcePoolBucketGroupQoSInfosParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListResourcePoolBucketGroupQoSInfosOps {
    /// List QoS configurations for all bucket-groups in a resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepoolbucketgroupqosinfos>
    fn list_resource_pool_bucket_group_qos_infos(
        &self,
        params: ListResourcePoolBucketGroupQoSInfosParams,
    ) -> impl Future<Output = Result<ListResourcePoolBucketGroupQoSInfosResult>>;
}

impl ListResourcePoolBucketGroupQoSInfosOps for Client {
    async fn list_resource_pool_bucket_group_qos_infos(
        &self,
        params: ListResourcePoolBucketGroupQoSInfosParams,
    ) -> Result<ListResourcePoolBucketGroupQoSInfosResult> {
        self.request(ListResourcePoolBucketGroupQoSInfos { params }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&ListResourcePoolBucketGroupQoSInfosParams::new("rp-for-ai")).unwrap();
        assert_eq!(q, "resourcePool=rp-for-ai&resourcePoolBucketGroupQoSInfo");
    }

    #[test]
    fn parse_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListResourcePoolBucketGroupQoSInfosResult>
  <ResourcePool>rp-for-ai</ResourcePool>
  <IsTruncated>false</IsTruncated>
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
  </ResourcePoolBucketGroupQoSInfo>
</ListResourcePoolBucketGroupQoSInfosResult>"#;
        let parsed: ListResourcePoolBucketGroupQoSInfosResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.bucket_group_qos_infos.len(), 1);
        assert_eq!(parsed.bucket_group_qos_infos[0].bucket_group, "test-group");
    }
}
