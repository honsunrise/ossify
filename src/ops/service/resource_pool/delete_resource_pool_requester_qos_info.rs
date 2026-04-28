//! DeleteResourcePoolRequesterQoSInfo: remove per-requester QoS against a resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteresourcepoolrequesterqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteResourcePoolRequesterQoSInfoParams {
    #[serde(rename = "requesterQosInfo")]
    pub(crate) requester_qos_info: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,
    #[serde(rename = "qosRequester")]
    pub qos_requester: String,
}

impl DeleteResourcePoolRequesterQoSInfoParams {
    pub fn new(resource_pool: impl Into<String>, qos_requester: impl Into<String>) -> Self {
        Self {
            requester_qos_info: OnlyKeyField,
            resource_pool: resource_pool.into(),
            qos_requester: qos_requester.into(),
        }
    }
}

pub struct DeleteResourcePoolRequesterQoSInfo {
    pub resource_pool: String,
    pub qos_requester: String,
}

impl Ops for DeleteResourcePoolRequesterQoSInfo {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteResourcePoolRequesterQoSInfoParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<DeleteResourcePoolRequesterQoSInfoParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteResourcePoolRequesterQoSInfoParams::new(
                self.resource_pool,
                self.qos_requester,
            )),
            ..Default::default()
        })
    }
}

pub trait DeleteResourcePoolRequesterQoSInfoOps {
    /// Delete per-requester QoS against a resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteresourcepoolrequesterqosinfo>
    fn delete_resource_pool_requester_qos_info(
        &self,
        resource_pool: impl Into<String>,
        qos_requester: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteResourcePoolRequesterQoSInfoOps for Client {
    async fn delete_resource_pool_requester_qos_info(
        &self,
        resource_pool: impl Into<String>,
        qos_requester: impl Into<String>,
    ) -> Result<()> {
        self.request(DeleteResourcePoolRequesterQoSInfo {
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
        let q = crate::ser::to_string(&DeleteResourcePoolRequesterQoSInfoParams::new("rp-for-ai", "300123"))
            .unwrap();
        assert_eq!(q, "qosRequester=300123&requesterQosInfo&resourcePool=rp-for-ai");
    }

    #[test]
    fn prepare_method() {
        let prepared = DeleteResourcePoolRequesterQoSInfo {
            resource_pool: "rp-for-ai".into(),
            qos_requester: "300123".into(),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::DELETE);
    }
}
