//! Options (CORS preflight) operation.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/options>

use std::collections::HashMap;
use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Deserialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// Response parsed from the preflight headers.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OptionsObjectResponse {
    #[serde(rename = "access-control-allow-origin", default)]
    pub access_control_allow_origin: Option<String>,
    #[serde(rename = "access-control-allow-methods", default)]
    pub access_control_allow_methods: Option<String>,
    #[serde(rename = "access-control-allow-headers", default)]
    pub access_control_allow_headers: Option<String>,
    #[serde(rename = "access-control-expose-headers", default)]
    pub access_control_expose_headers: Option<String>,
    #[serde(rename = "access-control-max-age", default)]
    pub access_control_max_age: Option<String>,
}

/// Request options for an OPTIONS preflight request.
#[derive(Debug, Clone, Default)]
pub struct OptionsObjectRequest {
    pub object_key: String,
    /// Required. The origin of the hypothetical cross-origin request.
    pub origin: String,
    /// Required. The method the actual request will use.
    pub access_control_request_method: String,
    /// Optional comma-separated list of headers.
    pub access_control_request_headers: Option<String>,
}

impl OptionsObjectRequest {
    fn to_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("origin"), self.origin.parse()?);
        headers.insert(
            HeaderName::from_static("access-control-request-method"),
            self.access_control_request_method.parse()?,
        );
        if let Some(h) = &self.access_control_request_headers {
            headers.insert(HeaderName::from_static("access-control-request-headers"), h.parse()?);
        }
        Ok(headers)
    }
}

pub struct OptionsObject {
    pub request: OptionsObjectRequest,
}

impl Ops for OptionsObject {
    type Response = HeaderResponseProcessor<OptionsObjectResponse>;
    type Body = NoneBody;
    type Query = HashMap<String, String>;

    fn prepare(self) -> Result<Prepared<HashMap<String, String>>> {
        Ok(Prepared {
            method: Method::OPTIONS,
            key: Some(self.request.object_key.clone()),
            headers: Some(self.request.to_headers()?),
            ..Default::default()
        })
    }
}

pub trait OptionsObjectOps {
    /// Send a CORS preflight (OPTIONS) request for an object.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/options>
    fn options_object(
        &self,
        request: OptionsObjectRequest,
    ) -> impl Future<Output = Result<OptionsObjectResponse>>;
}

impl OptionsObjectOps for Client {
    async fn options_object(&self, request: OptionsObjectRequest) -> Result<OptionsObjectResponse> {
        self.request(OptionsObject { request }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers_populated_correctly() {
        let req = OptionsObjectRequest {
            object_key: "file.txt".to_string(),
            origin: "http://www.example.com".to_string(),
            access_control_request_method: "PUT".to_string(),
            access_control_request_headers: Some("x-oss-test".to_string()),
        };
        let headers = req.to_headers().unwrap();
        assert_eq!(headers.get("origin").unwrap(), "http://www.example.com");
        assert_eq!(headers.get("access-control-request-method").unwrap(), "PUT");
        assert_eq!(headers.get("access-control-request-headers").unwrap(), "x-oss-test");
    }

    #[test]
    fn prepared_uses_options() {
        let prepared = OptionsObject {
            request: OptionsObjectRequest {
                object_key: "a".to_string(),
                origin: "http://x".to_string(),
                access_control_request_method: "GET".to_string(),
                access_control_request_headers: None,
            },
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::OPTIONS);
        assert_eq!(prepared.key.as_deref(), Some("a"));
    }
}
