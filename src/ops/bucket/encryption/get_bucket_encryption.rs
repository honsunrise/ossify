//! GetBucketEncryption.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketencryption>

use std::future::Future;

use http::Method;
use serde::Serialize;

#[allow(unused_imports)]
pub use super::put_bucket_encryption::ApplyServerSideEncryptionByDefault;
pub use super::put_bucket_encryption::ServerSideEncryptionRule;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketEncryptionParams {
    encryption: OnlyKeyField,
}

pub struct GetBucketEncryption;

impl Ops for GetBucketEncryption {
    type Response = BodyResponseProcessor<ServerSideEncryptionRule>;
    type Body = NoneBody;
    type Query = GetBucketEncryptionParams;

    fn prepare(self) -> Result<Prepared<GetBucketEncryptionParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketEncryptionParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketEncryptionOps {
    /// Query the bucket's default server-side encryption rule.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketencryption>
    fn get_bucket_encryption(&self) -> impl Future<Output = Result<ServerSideEncryptionRule>>;
}

impl GetBucketEncryptionOps for Client {
    async fn get_bucket_encryption(&self) -> Result<ServerSideEncryptionRule> {
        self.request(GetBucketEncryption).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::common::ServerSideEncryption;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketEncryptionParams::default()).unwrap(),
            "encryption"
        );
    }

    #[test]
    fn parse_kms_response() {
        let xml = r#"<ServerSideEncryptionRule>
  <ApplyServerSideEncryptionByDefault>
    <SSEAlgorithm>KMS</SSEAlgorithm>
    <KMSMasterKeyID>key-1</KMSMasterKeyID>
  </ApplyServerSideEncryptionByDefault>
</ServerSideEncryptionRule>"#;
        let parsed: ServerSideEncryptionRule = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            parsed.apply_server_side_encryption_by_default.sse_algorithm,
            ServerSideEncryption::Kms
        );
        assert_eq!(
            parsed
                .apply_server_side_encryption_by_default
                .kms_master_key_id
                .as_deref(),
            Some("key-1")
        );
    }
}
