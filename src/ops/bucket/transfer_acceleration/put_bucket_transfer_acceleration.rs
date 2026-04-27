//! PutBucketTransferAcceleration.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbuckettransferacceleration>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketTransferAccelerationParams {
    #[serde(rename = "transferAcceleration")]
    transfer_acceleration: OnlyKeyField,
}

/// Body for `PutBucketTransferAcceleration` / `GetBucketTransferAcceleration`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "TransferAccelerationConfiguration", rename_all = "PascalCase")]
pub struct TransferAccelerationConfiguration {
    pub enabled: bool,
}

impl TransferAccelerationConfiguration {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

pub struct PutBucketTransferAcceleration {
    pub config: TransferAccelerationConfiguration,
}

impl Ops for PutBucketTransferAcceleration {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<TransferAccelerationConfiguration>;
    type Query = PutBucketTransferAccelerationParams;

    fn prepare(
        self,
    ) -> Result<Prepared<PutBucketTransferAccelerationParams, TransferAccelerationConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketTransferAccelerationParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketTransferAccelerationOps {
    /// Enable / disable transfer acceleration for the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbuckettransferacceleration>
    fn put_bucket_transfer_acceleration(&self, enabled: bool) -> impl Future<Output = Result<()>>;
}

impl PutBucketTransferAccelerationOps for Client {
    async fn put_bucket_transfer_acceleration(&self, enabled: bool) -> Result<()> {
        self.request(PutBucketTransferAcceleration {
            config: TransferAccelerationConfiguration::new(enabled),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&PutBucketTransferAccelerationParams::default()).unwrap();
        assert_eq!(q, "transferAcceleration");
    }

    #[test]
    fn body_serializes() {
        let xml = quick_xml::se::to_string(&TransferAccelerationConfiguration::new(true)).unwrap();
        assert!(xml.contains("<TransferAccelerationConfiguration>"));
        assert!(xml.contains("<Enabled>true</Enabled>"));
    }

    #[test]
    fn prepared_uses_put() {
        let prepared = PutBucketTransferAcceleration {
            config: TransferAccelerationConfiguration::new(false),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::PUT);
    }
}
