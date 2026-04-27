//! PutBucketCors.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketcors>

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
pub struct PutBucketCorsParams {
    cors: OnlyKeyField,
}

/// A single `<CORSRule>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CorsRule {
    #[serde(rename = "AllowedOrigin", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_origins: Vec<String>,
    #[serde(rename = "AllowedMethod", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_methods: Vec<String>,
    #[serde(rename = "AllowedHeader", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_headers: Vec<String>,
    #[serde(rename = "ExposeHeader", default, skip_serializing_if = "Vec::is_empty")]
    pub expose_headers: Vec<String>,
    pub max_age_seconds: Option<u64>,
}

/// Root `<CORSConfiguration>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CORSConfiguration", rename_all = "PascalCase")]
pub struct CorsConfiguration {
    #[serde(rename = "CORSRule", default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<CorsRule>,
    /// When true, OSS always returns `Vary: Origin`.
    pub response_vary: Option<bool>,
}

pub struct PutBucketCors {
    pub config: CorsConfiguration,
}

impl Ops for PutBucketCors {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<CorsConfiguration>;
    type Query = PutBucketCorsParams;

    fn prepare(self) -> Result<Prepared<PutBucketCorsParams, CorsConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketCorsParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketCorsOps {
    /// Set the bucket's CORS rules. Overwrites any existing rules.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketcors>
    fn put_bucket_cors(&self, config: CorsConfiguration) -> impl Future<Output = Result<()>>;
}

impl PutBucketCorsOps for Client {
    async fn put_bucket_cors(&self, config: CorsConfiguration) -> Result<()> {
        self.request(PutBucketCors { config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&PutBucketCorsParams::default()).unwrap(), "cors");
    }

    #[test]
    fn parse_and_round_trip_rule() {
        let xml = r#"<CORSConfiguration>
<CORSRule>
  <AllowedOrigin>*</AllowedOrigin>
  <AllowedMethod>PUT</AllowedMethod>
  <AllowedMethod>GET</AllowedMethod>
  <AllowedHeader>Authorization</AllowedHeader>
</CORSRule>
<CORSRule>
  <AllowedOrigin>http://example.com</AllowedOrigin>
  <AllowedMethod>GET</AllowedMethod>
  <ExposeHeader>x-oss-test</ExposeHeader>
  <ExposeHeader>x-oss-test1</ExposeHeader>
  <MaxAgeSeconds>100</MaxAgeSeconds>
</CORSRule>
<ResponseVary>false</ResponseVary>
</CORSConfiguration>"#;
        let parsed: CorsConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.rules.len(), 2);
        assert_eq!(parsed.rules[0].allowed_methods, vec!["PUT".to_string(), "GET".to_string()]);
        assert_eq!(parsed.rules[1].max_age_seconds, Some(100));
        assert_eq!(parsed.response_vary, Some(false));
    }
}
