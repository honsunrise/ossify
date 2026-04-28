//! Callback helpers.
//!
//! OSS lets `PutObject`, `PostObject`, and `CompleteMultipartUpload` trigger a
//! POST callback to an application server after the upload succeeds. Callback
//! parameters are JSON objects that must be **Base64-encoded** and sent either:
//!
//! * as `x-oss-callback` and `x-oss-callback-var` HTTP headers (recommended,
//!   used by `PutObject` and `CompleteMultipartUpload`), or
//! * as form fields in a `PostObject` multipart body.
//!
//! This module models the JSON payloads, exposes encoders, and provides a
//! helper to inject callback headers into an arbitrary `HeaderMap`. It does
//! **not** define a separate `Ops` — callback is a rider on existing upload
//! APIs.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/callback>

use std::collections::BTreeMap;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use http::{HeaderMap, HeaderName};
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// `callbackBodyType` values supported by OSS.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CallbackBodyType {
    #[serde(rename = "application/x-www-form-urlencoded")]
    FormUrlEncoded,
    #[serde(rename = "application/json")]
    Json,
}

/// The `callback` parameter JSON payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallbackConfiguration {
    /// Application server URL(s); multiple URLs may be semicolon-separated.
    pub callback_url: String,
    /// Optional override of the `Host` header on the callback request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_host: Option<String>,
    /// Body template (form-url-encoded key=value pairs or JSON).
    pub callback_body: String,
    /// Body content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_body_type: Option<CallbackBodyType>,
    /// Whether to send SNI (HTTPS only).
    #[serde(rename = "callbackSNI", skip_serializing_if = "Option::is_none")]
    pub callback_sni: Option<bool>,
}

impl CallbackConfiguration {
    /// Create a minimal callback configuration.
    pub fn new(url: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            callback_url: url.into(),
            callback_host: None,
            callback_body: body.into(),
            callback_body_type: None,
            callback_sni: None,
        }
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.callback_host = Some(host.into());
        self
    }

    pub fn body_type(mut self, ty: CallbackBodyType) -> Self {
        self.callback_body_type = Some(ty);
        self
    }

    pub fn sni(mut self, sni: bool) -> Self {
        self.callback_sni = Some(sni);
        self
    }

    /// Serialize to JSON then Base64-encode (ready for the `x-oss-callback`
    /// header or `callback` form field).
    pub fn to_base64(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(BASE64.encode(json.as_bytes()))
    }
}

/// The `callback-var` parameter: custom user variables referenced as
/// `${x:foo}` from within `callback_body`.
///
/// All keys must start with `x:` per OSS spec.
#[derive(Debug, Clone, Default)]
pub struct CallbackVariables(BTreeMap<String, String>);

impl CallbackVariables {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a variable. The `x:` prefix is added automatically if missing.
    pub fn var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut k = key.into();
        if !k.starts_with("x:") {
            k = format!("x:{k}");
        }
        self.0.insert(k, value.into());
        self
    }

    /// Serialize to JSON then Base64-encode (ready for the
    /// `x-oss-callback-var` header).
    pub fn to_base64(&self) -> Result<Option<String>> {
        if self.0.is_empty() {
            return Ok(None);
        }
        let json = serde_json::to_string(&self.0)?;
        Ok(Some(BASE64.encode(json.as_bytes())))
    }

    /// Expose as raw `x:`-prefixed form fields for `PostObject`.
    pub fn as_form_fields(&self) -> &BTreeMap<String, String> {
        &self.0
    }
}

/// Append `x-oss-callback` and (optionally) `x-oss-callback-var` headers to
/// `headers`. Returns the mutated `HeaderMap`.
pub fn apply_callback_headers(
    mut headers: HeaderMap,
    callback: &CallbackConfiguration,
    variables: Option<&CallbackVariables>,
) -> Result<HeaderMap> {
    let cb_b64 = callback.to_base64()?;
    headers.insert(HeaderName::from_static("x-oss-callback"), cb_b64.parse()?);
    if let Some(vars) = variables
        && let Some(var_b64) = vars.to_base64()?
    {
        headers.insert(HeaderName::from_static("x-oss-callback-var"), var_b64.parse()?);
    }
    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callback_to_base64_roundtrip() {
        let cfg = CallbackConfiguration::new(
            "http://oss-demo.aliyuncs.com:23450",
            "bucket=${bucket}&object=${object}",
        )
        .body_type(CallbackBodyType::FormUrlEncoded)
        .sni(false);
        let b64 = cfg.to_base64().unwrap();
        let json = String::from_utf8(BASE64.decode(&b64).unwrap()).unwrap();
        assert!(json.contains("\"callbackUrl\":\"http://oss-demo.aliyuncs.com:23450\""));
        assert!(json.contains("\"callbackBodyType\":\"application/x-www-form-urlencoded\""));
        assert!(json.contains("\"callbackSNI\":false"));
    }

    #[test]
    fn test_callback_omits_optional_fields() {
        let cfg = CallbackConfiguration::new("http://example.com", "bucket=${bucket}");
        let b64 = cfg.to_base64().unwrap();
        let json = String::from_utf8(BASE64.decode(&b64).unwrap()).unwrap();
        assert!(!json.contains("callbackHost"));
        assert!(!json.contains("callbackBodyType"));
        assert!(!json.contains("callbackSNI"));
    }

    #[test]
    fn test_callback_variables_prefix() {
        let vars = CallbackVariables::new()
            .var("uid", "12345")
            .var("x:order_id", "67890");
        let b64 = vars.to_base64().unwrap().unwrap();
        let json = String::from_utf8(BASE64.decode(&b64).unwrap()).unwrap();
        assert!(json.contains("\"x:uid\":\"12345\""));
        assert!(json.contains("\"x:order_id\":\"67890\""));
    }

    #[test]
    fn test_callback_variables_empty_returns_none() {
        let vars = CallbackVariables::new();
        assert!(vars.to_base64().unwrap().is_none());
    }

    #[test]
    fn test_apply_callback_headers() {
        let cfg = CallbackConfiguration::new("http://example.com", "bucket=${bucket}");
        let vars = CallbackVariables::new().var("uid", "1");
        let headers = apply_callback_headers(HeaderMap::new(), &cfg, Some(&vars)).unwrap();
        assert!(headers.contains_key("x-oss-callback"));
        assert!(headers.contains_key("x-oss-callback-var"));
    }

    #[test]
    fn test_apply_callback_headers_without_vars() {
        let cfg = CallbackConfiguration::new("http://example.com", "bucket=${bucket}");
        let headers = apply_callback_headers(HeaderMap::new(), &cfg, None).unwrap();
        assert!(headers.contains_key("x-oss-callback"));
        assert!(!headers.contains_key("x-oss-callback-var"));
    }
}
