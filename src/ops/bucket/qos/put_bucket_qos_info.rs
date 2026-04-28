//! PutBucketQoSInfo: configure the bucket's total QoS (bandwidth cap).
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::QoSConfiguration;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`PutBucketQoSInfo`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketQoSInfoParams {
    #[serde(rename = "qosInfo")]
    qos_info: OnlyKeyField,
}

/// The `PutBucketQoSInfo` operation.
pub struct PutBucketQoSInfo {
    pub configuration: QoSConfiguration,
}

impl Ops for PutBucketQoSInfo {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<QoSConfiguration>;
    type Query = PutBucketQoSInfoParams;

    fn prepare(self) -> Result<Prepared<PutBucketQoSInfoParams, QoSConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketQoSInfoParams::default()),
            body: Some(self.configuration),
            ..Default::default()
        })
    }
}

/// Trait for the `PutBucketQoSInfo` operation.
pub trait PutBucketQoSInfoOps {
    /// Set the bucket's total QoS configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketqosinfo>
    fn put_bucket_qos_info(&self, configuration: QoSConfiguration) -> impl Future<Output = Result<()>>;
}

impl PutBucketQoSInfoOps for Client {
    async fn put_bucket_qos_info(&self, configuration: QoSConfiguration) -> Result<()> {
        self.request(PutBucketQoSInfo { configuration }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&PutBucketQoSInfoParams::default()).unwrap(), "qosInfo");
    }

    #[test]
    fn body_round_trip() {
        let cfg = QoSConfiguration::bandwidth(10, -1, -1, 10, -1, -1);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<TotalUploadBandwidth>10</TotalUploadBandwidth>"));
        let back: QoSConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}
