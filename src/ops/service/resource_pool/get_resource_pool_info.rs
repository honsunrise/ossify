//! GetResourcePoolInfo: fetch info and QoS configuration for a resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getresourcepoolinfo>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::QoSConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetResourcePoolInfoParams {
    #[serde(rename = "resourcePoolInfo")]
    pub(crate) resource_pool_info: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,
}

impl GetResourcePoolInfoParams {
    pub fn new(resource_pool: impl Into<String>) -> Self {
        Self {
            resource_pool_info: OnlyKeyField,
            resource_pool: resource_pool.into(),
        }
    }
}

/// Response body of [`GetResourcePoolInfo`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename = "GetResourcePoolInfoResponse", rename_all = "PascalCase")]
pub struct GetResourcePoolInfoResult {
    pub region: Option<String>,
    pub name: String,
    pub owner: Option<String>,
    pub create_time: Option<String>,
    #[serde(rename = "QoSConfiguration")]
    pub qos_configuration: QoSConfiguration,
}

pub struct GetResourcePoolInfo {
    pub resource_pool: String,
}

impl Ops for GetResourcePoolInfo {
    type Response = BodyResponseProcessor<GetResourcePoolInfoResult>;
    type Body = NoneBody;
    type Query = GetResourcePoolInfoParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<GetResourcePoolInfoParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetResourcePoolInfoParams::new(self.resource_pool)),
            ..Default::default()
        })
    }
}

pub trait GetResourcePoolInfoOps {
    /// Fetch info and QoS configuration for a resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getresourcepoolinfo>
    fn get_resource_pool_info(
        &self,
        resource_pool: impl Into<String>,
    ) -> impl Future<Output = Result<GetResourcePoolInfoResult>>;
}

impl GetResourcePoolInfoOps for Client {
    async fn get_resource_pool_info(
        &self,
        resource_pool: impl Into<String>,
    ) -> Result<GetResourcePoolInfoResult> {
        self.request(GetResourcePoolInfo {
            resource_pool: resource_pool.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&GetResourcePoolInfoParams::new("rp-for-ai")).unwrap();
        assert_eq!(q, "resourcePool=rp-for-ai&resourcePoolInfo");
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<GetResourcePoolInfo as Ops>::USE_BUCKET);
    }

    #[test]
    fn parse_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<GetResourcePoolInfoResponse>
  <Region>oss-cn-hangzhou</Region>
  <Name>rp-for-ai</Name>
  <Owner>103xxxx</Owner>
  <CreateTime>2024-11-29T08:42:32.000Z</CreateTime>
  <QoSConfiguration>
    <TotalUploadBandwidth>10</TotalUploadBandwidth>
    <IntranetUploadBandwidth>-1</IntranetUploadBandwidth>
    <ExtranetUploadBandwidth>-1</ExtranetUploadBandwidth>
    <TotalDownloadBandwidth>10</TotalDownloadBandwidth>
    <IntranetDownloadBandwidth>-1</IntranetDownloadBandwidth>
    <ExtranetDownloadBandwidth>-1</ExtranetDownloadBandwidth>
  </QoSConfiguration>
</GetResourcePoolInfoResponse>"#;
        let parsed: GetResourcePoolInfoResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.name, "rp-for-ai");
        assert_eq!(parsed.qos_configuration.total_upload_bandwidth, 10);
    }
}
