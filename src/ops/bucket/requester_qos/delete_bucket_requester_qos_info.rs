//! DeleteBucketRequesterQoSInfo: remove a UID-specific QoS from the bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketrequesterqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteBucketRequesterQoSInfoParams {
    #[serde(rename = "requesterQosInfo")]
    pub(crate) requester_qos_info: OnlyKeyField,
    #[serde(rename = "qosRequester")]
    pub qos_requester: String,
}

impl DeleteBucketRequesterQoSInfoParams {
    pub fn new(qos_requester: impl Into<String>) -> Self {
        Self {
            requester_qos_info: OnlyKeyField,
            qos_requester: qos_requester.into(),
        }
    }
}

pub struct DeleteBucketRequesterQoSInfo {
    pub qos_requester: String,
}

impl Ops for DeleteBucketRequesterQoSInfo {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketRequesterQoSInfoParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketRequesterQoSInfoParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketRequesterQoSInfoParams::new(self.qos_requester)),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketRequesterQoSInfoOps {
    /// Delete the bucket's per-requester QoS for a given UID.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketrequesterqosinfo>
    fn delete_bucket_requester_qos_info(
        &self,
        qos_requester: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketRequesterQoSInfoOps for Client {
    async fn delete_bucket_requester_qos_info(&self, qos_requester: impl Into<String>) -> Result<()> {
        self.request(DeleteBucketRequesterQoSInfo {
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
        let q = crate::ser::to_string(&DeleteBucketRequesterQoSInfoParams::new("300123")).unwrap();
        assert_eq!(q, "qosRequester=300123&requesterQosInfo");
    }

    #[test]
    fn prepare_method() {
        let prepared = DeleteBucketRequesterQoSInfo {
            qos_requester: "300123".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::DELETE);
    }
}
