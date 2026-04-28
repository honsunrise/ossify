//! GetBucketRequesterQoSInfo: fetch a UID-specific QoS for the bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketrequesterqosinfo>

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
pub struct GetBucketRequesterQoSInfoParams {
    #[serde(rename = "requesterQosInfo")]
    pub(crate) requester_qos_info: OnlyKeyField,
    #[serde(rename = "qosRequester")]
    pub qos_requester: String,
}

impl GetBucketRequesterQoSInfoParams {
    pub fn new(qos_requester: impl Into<String>) -> Self {
        Self {
            requester_qos_info: OnlyKeyField,
            qos_requester: qos_requester.into(),
        }
    }
}

pub struct GetBucketRequesterQoSInfo {
    pub qos_requester: String,
}

impl Ops for GetBucketRequesterQoSInfo {
    type Response = BodyResponseProcessor<RequesterQoSInfo>;
    type Body = NoneBody;
    type Query = GetBucketRequesterQoSInfoParams;

    fn prepare(self) -> Result<Prepared<GetBucketRequesterQoSInfoParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketRequesterQoSInfoParams::new(self.qos_requester)),
            ..Default::default()
        })
    }
}

pub trait GetBucketRequesterQoSInfoOps {
    /// Fetch the bucket's per-requester QoS for a given UID.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketrequesterqosinfo>
    fn get_bucket_requester_qos_info(
        &self,
        qos_requester: impl Into<String>,
    ) -> impl Future<Output = Result<RequesterQoSInfo>>;
}

impl GetBucketRequesterQoSInfoOps for Client {
    async fn get_bucket_requester_qos_info(
        &self,
        qos_requester: impl Into<String>,
    ) -> Result<RequesterQoSInfo> {
        self.request(GetBucketRequesterQoSInfo {
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
        let q = crate::ser::to_string(&GetBucketRequesterQoSInfoParams::new("300123")).unwrap();
        assert_eq!(q, "qosRequester=300123&requesterQosInfo");
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
  </QoSConfiguration>
</RequesterQoSInfo>"#;
        let info: RequesterQoSInfo = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(info.requester, "300123");
        assert_eq!(info.qos_configuration.total_upload_bandwidth, 10);
    }
}
