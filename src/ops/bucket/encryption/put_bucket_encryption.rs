//! PutBucketEncryption.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketencryption>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::ServerSideEncryption;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketEncryptionParams {
    encryption: OnlyKeyField,
}

/// `<ApplyServerSideEncryptionByDefault>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyServerSideEncryptionByDefault {
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: ServerSideEncryption,
    /// Data-encryption algorithm. Only `SM4` is valid today and only when
    /// `sse_algorithm` is `Kms`.
    #[serde(rename = "KMSDataEncryption")]
    pub kms_data_encryption: Option<String>,
    /// Customer master key ID. Only applicable when `sse_algorithm` is `Kms`.
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_master_key_id: Option<String>,
}

/// Root `<ServerSideEncryptionRule>` element.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ServerSideEncryptionRule", rename_all = "PascalCase")]
pub struct ServerSideEncryptionRule {
    pub apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault,
}

pub struct PutBucketEncryption {
    pub rule: ServerSideEncryptionRule,
}

impl Ops for PutBucketEncryption {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<ServerSideEncryptionRule>;
    type Query = PutBucketEncryptionParams;

    fn prepare(self) -> Result<Prepared<PutBucketEncryptionParams, ServerSideEncryptionRule>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketEncryptionParams::default()),
            body: Some(self.rule),
            ..Default::default()
        })
    }
}

pub trait PutBucketEncryptionOps {
    /// Configure the default server-side encryption rule for the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketencryption>
    fn put_bucket_encryption(&self, rule: ServerSideEncryptionRule) -> impl Future<Output = Result<()>>;
}

impl PutBucketEncryptionOps for Client {
    async fn put_bucket_encryption(&self, rule: ServerSideEncryptionRule) -> Result<()> {
        self.request(PutBucketEncryption { rule }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutBucketEncryptionParams::default()).unwrap(),
            "encryption"
        );
    }

    #[test]
    fn body_round_trip_kms() {
        let rule = ServerSideEncryptionRule {
            apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
                sse_algorithm: ServerSideEncryption::Kms,
                kms_data_encryption: Some("SM4".to_string()),
                kms_master_key_id: Some("key-1".to_string()),
            },
        };
        let xml = quick_xml::se::to_string(&rule).unwrap();
        assert!(xml.contains("<SSEAlgorithm>KMS</SSEAlgorithm>"));
        assert!(xml.contains("<KMSDataEncryption>SM4</KMSDataEncryption>"));
        let back: ServerSideEncryptionRule = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, rule);
    }

    #[test]
    fn body_round_trip_aes256() {
        let rule = ServerSideEncryptionRule {
            apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
                sse_algorithm: ServerSideEncryption::Aes256,
                kms_data_encryption: None,
                kms_master_key_id: None,
            },
        };
        let xml = quick_xml::se::to_string(&rule).unwrap();
        assert!(xml.contains("<SSEAlgorithm>AES256</SSEAlgorithm>"));
    }
}
