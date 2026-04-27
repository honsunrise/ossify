//! PutBucketHttpsConfig.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbuckethttpsconfig>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketHttpsConfigParams {
    #[serde(rename = "httpsConfig")]
    https_config: OnlyKeyField,
}

/// `<TLS>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tls {
    pub enable: bool,
    #[serde(rename = "TLSVersion", default, skip_serializing_if = "Vec::is_empty")]
    pub tls_versions: Vec<String>,
}

/// `<CipherSuite>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CipherSuite {
    pub enable: Option<bool>,
    pub strong_cipher_suite: Option<bool>,
    #[serde(rename = "CustomCipherSuite", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_cipher_suites: Vec<String>,
    #[serde(
        rename = "TLS13CustomCipherSuite",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tls13_custom_cipher_suites: Vec<String>,
}

/// Root `<HttpsConfiguration>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "HttpsConfiguration", rename_all = "PascalCase")]
pub struct HttpsConfiguration {
    #[serde(rename = "TLS")]
    pub tls: Tls,
    pub cipher_suite: Option<CipherSuite>,
}

pub struct PutBucketHttpsConfig {
    pub config: HttpsConfiguration,
}

impl Ops for PutBucketHttpsConfig {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<HttpsConfiguration>;
    type Query = PutBucketHttpsConfigParams;

    fn prepare(self) -> Result<Prepared<PutBucketHttpsConfigParams, HttpsConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketHttpsConfigParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketHttpsConfigOps {
    /// Configure the bucket's TLS versions and cipher-suite policy.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbuckethttpsconfig>
    fn put_bucket_https_config(&self, config: HttpsConfiguration) -> impl Future<Output = Result<()>>;
}

impl PutBucketHttpsConfigOps for Client {
    async fn put_bucket_https_config(&self, config: HttpsConfiguration) -> Result<()> {
        self.request(PutBucketHttpsConfig { config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutBucketHttpsConfigParams::default()).unwrap(),
            "httpsConfig"
        );
    }

    #[test]
    fn body_round_trip() {
        let cfg = HttpsConfiguration {
            tls: Tls {
                enable: true,
                tls_versions: vec!["TLSv1.2".to_string(), "TLSv1.3".to_string()],
            },
            cipher_suite: Some(CipherSuite {
                enable: Some(true),
                strong_cipher_suite: Some(false),
                custom_cipher_suites: vec!["ECDHE-ECDSA-AES128-SHA256".to_string()],
                tls13_custom_cipher_suites: vec!["ECDHE-ECDSA-AES256-CCM8".to_string()],
            }),
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<TLSVersion>TLSv1.2</TLSVersion>"));
        let back: HttpsConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}
