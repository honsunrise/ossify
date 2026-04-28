//! PutResourcePoolRequesterQoSInfo: configure per-UID QoS against a resource pool.
//!
//! This differs from the bucket-level variant in two ways:
//!
//! - The URL carries `resourcePool=<name>` in addition to `qosRequester=<uid>`.
//! - The `<QoSConfiguration>` body may additionally contain
//!   `TotalQps`/`IntranetQps`/`ExtranetQps` fields.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putresourcepoolrequesterqosinfo>

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
pub struct PutResourcePoolRequesterQoSInfoParams {
    #[serde(rename = "requesterQosInfo")]
    pub(crate) requester_qos_info: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,
    #[serde(rename = "qosRequester")]
    pub qos_requester: String,
}

impl PutResourcePoolRequesterQoSInfoParams {
    pub fn new(resource_pool: impl Into<String>, qos_requester: impl Into<String>) -> Self {
        Self {
            requester_qos_info: OnlyKeyField,
            resource_pool: resource_pool.into(),
            qos_requester: qos_requester.into(),
        }
    }
}

pub struct PutResourcePoolRequesterQoSInfo {
    pub resource_pool: String,
    pub qos_requester: String,
    pub configuration: QoSConfiguration,
}

impl Ops for PutResourcePoolRequesterQoSInfo {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<QoSConfiguration>;
    type Query = PutResourcePoolRequesterQoSInfoParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<PutResourcePoolRequesterQoSInfoParams, QoSConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutResourcePoolRequesterQoSInfoParams::new(
                self.resource_pool,
                self.qos_requester,
            )),
            body: Some(self.configuration),
            ..Default::default()
        })
    }
}

pub trait PutResourcePoolRequesterQoSInfoOps {
    /// Put per-requester QoS against a resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putresourcepoolrequesterqosinfo>
    fn put_resource_pool_requester_qos_info(
        &self,
        resource_pool: impl Into<String>,
        qos_requester: impl Into<String>,
        configuration: QoSConfiguration,
    ) -> impl Future<Output = Result<()>>;
}

impl PutResourcePoolRequesterQoSInfoOps for Client {
    async fn put_resource_pool_requester_qos_info(
        &self,
        resource_pool: impl Into<String>,
        qos_requester: impl Into<String>,
        configuration: QoSConfiguration,
    ) -> Result<()> {
        self.request(PutResourcePoolRequesterQoSInfo {
            resource_pool: resource_pool.into(),
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
        let q = crate::ser::to_string(&PutResourcePoolRequesterQoSInfoParams::new("rp-for-ai", "300123"))
            .unwrap();
        assert_eq!(q, "qosRequester=300123&requesterQosInfo&resourcePool=rp-for-ai");
    }

    #[test]
    fn body_round_trip_with_qps() {
        let cfg = QoSConfiguration::bandwidth(10, -1, -1, 10, -1, -1).with_qps(-1, -1, -1);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<TotalQps>-1</TotalQps>"));
        let back: QoSConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<PutResourcePoolRequesterQoSInfo as Ops>::USE_BUCKET);
    }
}
