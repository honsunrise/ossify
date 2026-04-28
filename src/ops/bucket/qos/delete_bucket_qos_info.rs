//! DeleteBucketQosInfo: remove the bucket's QoS configuration.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketqosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketQoSInfoParams {
    #[serde(rename = "qosInfo")]
    qos_info: OnlyKeyField,
}

pub struct DeleteBucketQoSInfo;

impl Ops for DeleteBucketQoSInfo {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketQoSInfoParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketQoSInfoParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketQoSInfoParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketQoSInfoOps {
    /// Delete the bucket's QoS configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketqosinfo>
    fn delete_bucket_qos_info(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketQoSInfoOps for Client {
    async fn delete_bucket_qos_info(&self) -> Result<()> {
        self.request(DeleteBucketQoSInfo).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&DeleteBucketQoSInfoParams::default()).unwrap(), "qosInfo");
    }

    #[test]
    fn prepare_method() {
        let prepared = DeleteBucketQoSInfo.prepare().unwrap();
        assert_eq!(prepared.method, Method::DELETE);
    }
}
