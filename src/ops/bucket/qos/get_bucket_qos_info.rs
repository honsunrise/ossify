//! GetBucketQoSInfo: fetch the bucket's QoS configuration.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::QoSConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketQoSInfoParams {
    #[serde(rename = "qosInfo")]
    qos_info: OnlyKeyField,
}

pub struct GetBucketQoSInfo;

impl Ops for GetBucketQoSInfo {
    type Response = BodyResponseProcessor<QoSConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketQoSInfoParams;

    fn prepare(self) -> Result<Prepared<GetBucketQoSInfoParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketQoSInfoParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketQoSInfoOps {
    /// Fetch the bucket's QoS configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketqosinfo>
    fn get_bucket_qos_info(&self) -> impl Future<Output = Result<QoSConfiguration>>;
}

impl GetBucketQoSInfoOps for Client {
    async fn get_bucket_qos_info(&self) -> Result<QoSConfiguration> {
        self.request(GetBucketQoSInfo).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetBucketQoSInfoParams::default()).unwrap(), "qosInfo");
    }

    #[test]
    fn parse_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<QoSConfiguration>
  <TotalUploadBandwidth>10</TotalUploadBandwidth>
  <IntranetUploadBandwidth>-1</IntranetUploadBandwidth>
  <ExtranetUploadBandwidth>-1</ExtranetUploadBandwidth>
  <TotalDownloadBandwidth>10</TotalDownloadBandwidth>
  <IntranetDownloadBandwidth>-1</IntranetDownloadBandwidth>
  <ExtranetDownloadBandwidth>-1</ExtranetDownloadBandwidth>
</QoSConfiguration>"#;
        let cfg: QoSConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(cfg.total_upload_bandwidth, 10);
        assert_eq!(cfg.intranet_upload_bandwidth, -1);
        assert!(cfg.total_qps.is_none());
    }
}
