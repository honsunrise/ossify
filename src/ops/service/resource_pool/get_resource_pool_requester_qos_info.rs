//! GetResourcePoolRequesterQoSInfo: fetch per-requester QoS against a resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getresourcepoolrequesterqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::RequesterQoSInfo;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetResourcePoolRequesterQoSInfoParams {
    #[serde(rename = "requesterQosInfo")]
    pub(crate) requester_qos_info: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,
    #[serde(rename = "qosRequester")]
    pub qos_requester: String,
}

impl GetResourcePoolRequesterQoSInfoParams {
    pub fn new(resource_pool: impl Into<String>, qos_requester: impl Into<String>) -> Self {
        Self {
            requester_qos_info: OnlyKeyField,
            resource_pool: resource_pool.into(),
            qos_requester: qos_requester.into(),
        }
    }
}

pub struct GetResourcePoolRequesterQoSInfo {
    pub resource_pool: String,
    pub qos_requester: String,
}

impl Ops for GetResourcePoolRequesterQoSInfo {
    type Response = BodyResponseProcessor<RequesterQoSInfo>;
    type Body = NoneBody;
    type Query = GetResourcePoolRequesterQoSInfoParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<GetResourcePoolRequesterQoSInfoParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetResourcePoolRequesterQoSInfoParams::new(
                self.resource_pool,
                self.qos_requester,
            )),
            ..Default::default()
        })
    }
}

pub trait GetResourcePoolRequesterQoSInfoOps {
    /// Fetch per-requester QoS against a resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getresourcepoolrequesterqosinfo>
    fn get_resource_pool_requester_qos_info(
        &self,
        resource_pool: impl Into<String>,
        qos_requester: impl Into<String>,
    ) -> impl Future<Output = Result<RequesterQoSInfo>>;
}

impl GetResourcePoolRequesterQoSInfoOps for Client {
    async fn get_resource_pool_requester_qos_info(
        &self,
        resource_pool: impl Into<String>,
        qos_requester: impl Into<String>,
    ) -> Result<RequesterQoSInfo> {
        self.request(GetResourcePoolRequesterQoSInfo {
            resource_pool: resource_pool.into(),
            qos_requester: qos_requester.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&GetResourcePoolRequesterQoSInfoParams::new("rp-for-ai", "300123"))
            .unwrap();
        assert_eq!(q, "qosRequester=300123&requesterQosInfo&resourcePool=rp-for-ai");
    }

    #[test]
    fn parse_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<RequesterQoSInfo>
  <Requester>300123</Requester>
  <QoSConfiguration>
    <TotalUploadBandwidth>10</TotalUploadBandwidth>
    <IntranetUploadBandwidth>-1</IntranetUploadBandwidth>
    <ExtranetUploadBandwidth>-1</ExtranetUploadBandwidth>
    <TotalDownloadBandwidth>10</TotalDownloadBandwidth>
    <IntranetDownloadBandwidth>-1</IntranetDownloadBandwidth>
    <ExtranetDownloadBandwidth>-1</ExtranetDownloadBandwidth>
    <TotalQps>-1</TotalQps>
    <IntranetQps>-1</IntranetQps>
    <ExtranetQps>-1</ExtranetQps>
  </QoSConfiguration>
</RequesterQoSInfo>"#;
        let info: RequesterQoSInfo = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(info.requester, "300123");
        assert_eq!(info.qos_configuration.total_qps, Some(-1));
    }
}
