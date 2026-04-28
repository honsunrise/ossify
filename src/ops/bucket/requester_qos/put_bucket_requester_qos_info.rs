//! PutBucketRequesterQoSInfo: set a UID-specific QoS for the current bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketrequesterqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::QoSConfiguration;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`PutBucketRequesterQoSInfo`].
#[derive(Debug, Clone, Serialize)]
pub struct PutBucketRequesterQoSInfoParams {
    #[serde(rename = "requesterQosInfo")]
    pub(crate) requester_qos_info: OnlyKeyField,
    /// Target Alibaba Cloud UID whose QoS is being configured.
    #[serde(rename = "qosRequester")]
    pub qos_requester: String,
}

impl PutBucketRequesterQoSInfoParams {
    pub fn new(qos_requester: impl Into<String>) -> Self {
        Self {
            requester_qos_info: OnlyKeyField,
            qos_requester: qos_requester.into(),
        }
    }
}

/// The `PutBucketRequesterQoSInfo` operation.
pub struct PutBucketRequesterQoSInfo {
    pub qos_requester: String,
    pub configuration: QoSConfiguration,
}

impl Ops for PutBucketRequesterQoSInfo {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<QoSConfiguration>;
    type Query = PutBucketRequesterQoSInfoParams;

    fn prepare(self) -> Result<Prepared<PutBucketRequesterQoSInfoParams, QoSConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketRequesterQoSInfoParams::new(self.qos_requester)),
            body: Some(self.configuration),
            ..Default::default()
        })
    }
}

pub trait PutBucketRequesterQoSInfoOps {
    /// Set the per-requester QoS for the given UID on the current bucket.
    ///
    /// Bucket-level requester bandwidth floor is 5 Gbit/s per field.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketrequesterqosinfo>
    fn put_bucket_requester_qos_info(
        &self,
        qos_requester: impl Into<String>,
        configuration: QoSConfiguration,
    ) -> impl Future<Output = Result<()>>;
}

impl PutBucketRequesterQoSInfoOps for Client {
    async fn put_bucket_requester_qos_info(
        &self,
        qos_requester: impl Into<String>,
        configuration: QoSConfiguration,
    ) -> Result<()> {
        self.request(PutBucketRequesterQoSInfo {
            qos_requester: qos_requester.into(),
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
        let q = crate::ser::to_string(&PutBucketRequesterQoSInfoParams::new("300123")).unwrap();
        // Serializer sorts alphabetically: qosRequester before requesterQosInfo.
        assert_eq!(q, "qosRequester=300123&requesterQosInfo");
    }

    #[test]
    fn body_round_trip() {
        let cfg = QoSConfiguration::bandwidth(10, -1, -1, 10, -1, -1);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        let back: QoSConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}
