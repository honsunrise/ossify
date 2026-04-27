//! GetBucketTransferAcceleration.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbuckettransferacceleration>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_transfer_acceleration::TransferAccelerationConfiguration;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketTransferAccelerationParams {
    #[serde(rename = "transferAcceleration")]
    transfer_acceleration: OnlyKeyField,
}

pub struct GetBucketTransferAcceleration;

impl Ops for GetBucketTransferAcceleration {
    type Response = BodyResponseProcessor<TransferAccelerationConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketTransferAccelerationParams;

    fn prepare(self) -> Result<Prepared<GetBucketTransferAccelerationParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketTransferAccelerationParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketTransferAccelerationOps {
    /// Query the transfer acceleration state of the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbuckettransferacceleration>
    fn get_bucket_transfer_acceleration(
        &self,
    ) -> impl Future<Output = Result<TransferAccelerationConfiguration>>;
}

impl GetBucketTransferAccelerationOps for Client {
    async fn get_bucket_transfer_acceleration(&self) -> Result<TransferAccelerationConfiguration> {
        self.request(GetBucketTransferAcceleration).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&GetBucketTransferAccelerationParams::default()).unwrap();
        assert_eq!(q, "transferAcceleration");
    }

    #[test]
    fn parse_enabled() {
        let xml = r#"<TransferAccelerationConfiguration><Enabled>true</Enabled></TransferAccelerationConfiguration>"#;
        let parsed: TransferAccelerationConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.enabled);
    }

    #[test]
    fn parse_disabled() {
        let xml = r#"<TransferAccelerationConfiguration><Enabled>false</Enabled></TransferAccelerationConfiguration>"#;
        let parsed: TransferAccelerationConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert!(!parsed.enabled);
    }
}
