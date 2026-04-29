//! PostObject operation.
//!
//! `PostObject` is fundamentally different from the other OSS APIs: it is a
//! browser-side HTML-form upload (`multipart/form-data`) where the request
//! carries its own authentication material in the form fields (`policy`,
//! `x-oss-signature`, etc.). Because the form can't be sent from a SDK in the
//! same way as `PutObject`, this module provides helpers to **generate the
//! signed form fields** on the server side so they can be embedded in an HTML
//! page or returned as JSON to a client.
//!
//! Two signing variants are supported: V4 (recommended) and V1 (legacy).
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/postobject>

use std::collections::BTreeMap;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::credential::hmac256;
use crate::error::Result;

/// Raw POST policy document (per OSS spec).
///
/// The policy is JSON that declares `expiration` (ISO-8601 UTC) and a list of
/// `conditions` that the uploaded form must satisfy. We expose it as an opaque
/// JSON string so callers can freely add conditions; see the official docs for
/// the condition syntax.
///
/// # Example
///
/// ```ignore
/// use ossify::ops::object::base::PostPolicy;
/// let policy = PostPolicy::new()
///     .expiration_rfc3339("2030-01-01T00:00:00Z")
///     .condition(serde_json::json!({"bucket": "examplebucket"}))
///     .condition(serde_json::json!(["starts-with", "$key", "user/eric/"]))
///     .condition(serde_json::json!(["content-length-range", 0, 1048576]));
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostPolicy {
    pub expiration: String,
    pub conditions: Vec<serde_json::Value>,
}

impl PostPolicy {
    /// Create an empty policy.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set expiration time to the given RFC-3339/ISO-8601 UTC string.
    pub fn expiration_rfc3339(mut self, rfc3339: impl Into<String>) -> Self {
        self.expiration = rfc3339.into();
        self
    }

    /// Set expiration to `n` seconds from now (UTC).
    pub fn expire_in(mut self, secs: i64) -> Self {
        let ts = Timestamp::now()
            .checked_add(jiff::Span::new().seconds(secs))
            .unwrap_or_else(|_| Timestamp::now());
        self.expiration = ts.to_string();
        self
    }

    /// Append a condition to the policy.
    pub fn condition(mut self, cond: serde_json::Value) -> Self {
        self.conditions.push(cond);
        self
    }

    /// Serialize to JSON and then Base64 encode.
    pub fn to_base64(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(BASE64.encode(json.as_bytes()))
    }
}

/// Signed form fields returned by [`sign_post_object_v4`].
///
/// These fields should be inserted into the HTML form (or the POST multipart
/// payload) exactly as returned. The `file` field (uploaded content) must be
/// the **last** field in the form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostObjectFormFields {
    /// Base64-encoded policy document.
    pub policy: String,
    /// `x-oss-signature-version` form field.
    pub signature_version: String,
    /// `x-oss-credential` form field (V4 only).
    pub credential: String,
    /// `x-oss-date` form field (V4 only, yyyymmddThhmmssZ).
    pub date: String,
    /// `x-oss-signature` form field.
    pub signature: String,
    /// Optional security token (STS). If present, add as `x-oss-security-token`.
    pub security_token: Option<String>,
}

/// Inputs for [`sign_post_object_v4`].
#[derive(Debug, Clone)]
pub struct PostObjectSignV4Input<'a> {
    /// Access Key ID.
    pub access_key_id: &'a str,
    /// Access Key Secret.
    pub access_key_secret: &'a str,
    /// Optional STS security token.
    pub security_token: Option<&'a str>,
    /// Region identifier, e.g. `cn-hangzhou`.
    pub region: &'a str,
    /// Product identifier; for OSS this is always `"oss"`.
    pub product: &'a str,
    /// Policy to sign.
    pub policy: &'a PostPolicy,
    /// Signing timestamp.
    pub timestamp: Timestamp,
}

/// Sign a POST object policy using OSS V4 signature.
///
/// The returned fields (together with the key, file, and other user fields)
/// should be submitted as `multipart/form-data` to `https://<bucket>.<host>/`.
pub fn sign_post_object_v4(input: PostObjectSignV4Input<'_>) -> Result<PostObjectFormFields> {
    // yyyymmdd
    let date_short = input.timestamp.strftime("%Y%m%d").to_string();
    // yyyymmddThhmmssZ
    let date_long = input.timestamp.strftime("%Y%m%dT%H%M%SZ").to_string();

    let credential = format!(
        "{}/{date_short}/{}/{}/aliyun_v4_request",
        input.access_key_id, input.region, input.product
    );

    // Add required V4 conditions into the policy if not already present.
    let mut policy = input.policy.clone();
    policy
        .conditions
        .push(serde_json::json!({"x-oss-signature-version": "OSS4-HMAC-SHA256"}));
    policy
        .conditions
        .push(serde_json::json!({"x-oss-credential": credential}));
    policy
        .conditions
        .push(serde_json::json!({"x-oss-date": date_long}));
    if let Some(token) = input.security_token {
        policy
            .conditions
            .push(serde_json::json!({"x-oss-security-token": token}));
    }

    let policy_b64 = policy.to_base64()?;

    // Signing key = HMAC("aliyun_v4" + secret, date) -> region -> product -> "aliyun_v4_request"
    let secret = format!("aliyun_v4{}", input.access_key_secret);
    let k_date = hmac256(secret.as_bytes(), &date_short)?;
    let k_region = hmac256(&k_date, input.region)?;
    let k_product = hmac256(&k_region, input.product)?;
    let signing_key = hmac256(&k_product, "aliyun_v4_request")?;

    // OSS V4 POST signature: HMAC-SHA256(signing_key, base64_policy) -> hex
    let sig_bytes = hmac256(&signing_key, &policy_b64)?;
    let signature = hex::encode(sig_bytes);

    Ok(PostObjectFormFields {
        policy: policy_b64,
        signature_version: "OSS4-HMAC-SHA256".into(),
        credential,
        date: date_long,
        signature,
        security_token: input.security_token.map(str::to_string),
    })
}

/// Inputs for [`sign_post_object_v1`] (legacy signature).
#[derive(Debug, Clone)]
pub struct PostObjectSignV1Input<'a> {
    pub access_key_id: &'a str,
    pub access_key_secret: &'a str,
    pub security_token: Option<&'a str>,
    pub policy: &'a PostPolicy,
}

/// V1 POST signature: `base64(hmac_sha1(secret, base64_policy))`.
///
/// Returned `credential` and `date` fields are empty for V1 (not used).
pub fn sign_post_object_v1(input: PostObjectSignV1Input<'_>) -> Result<PostObjectFormFields> {
    use hmac::{Hmac, KeyInit, Mac};
    use sha1::Sha1;

    let policy_b64 = input.policy.to_base64()?;

    let mut mac = Hmac::<Sha1>::new_from_slice(input.access_key_secret.as_bytes())
        .map_err(|e| anyhow::anyhow!("invalid HMAC key length: {e}"))?;
    mac.update(policy_b64.as_bytes());
    let sig = mac.finalize().into_bytes();
    let signature = BASE64.encode(sig);

    Ok(PostObjectFormFields {
        policy: policy_b64,
        signature_version: String::new(),
        credential: input.access_key_id.to_string(),
        date: String::new(),
        signature,
        security_token: input.security_token.map(str::to_string),
    })
}

/// Assemble the final key/value form fields (including key and user-meta) for
/// the HTML form. The `file` field must still be appended by the caller as the
/// last form part.
pub fn build_form_fields(
    signed: &PostObjectFormFields,
    key: impl Into<String>,
    extra: BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    fields.insert("key".to_string(), key.into());
    fields.insert("policy".to_string(), signed.policy.clone());
    if !signed.signature_version.is_empty() {
        fields.insert("x-oss-signature-version".to_string(), signed.signature_version.clone());
        fields.insert("x-oss-credential".to_string(), signed.credential.clone());
        fields.insert("x-oss-date".to_string(), signed.date.clone());
        fields.insert("x-oss-signature".to_string(), signed.signature.clone());
    } else {
        // V1
        fields.insert("OSSAccessKeyId".to_string(), signed.credential.clone());
        fields.insert("Signature".to_string(), signed.signature.clone());
    }
    if let Some(token) = &signed.security_token {
        fields.insert("x-oss-security-token".to_string(), token.clone());
    }
    fields.extend(extra);
    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_to_base64() {
        let policy = PostPolicy::new()
            .expiration_rfc3339("2030-01-01T00:00:00Z")
            .condition(serde_json::json!({"bucket": "examplebucket"}));
        let b64 = policy.to_base64().unwrap();
        let json = String::from_utf8(BASE64.decode(&b64).unwrap()).unwrap();
        assert!(json.contains("\"expiration\":\"2030-01-01T00:00:00Z\""));
        assert!(json.contains("\"bucket\":\"examplebucket\""));
    }

    #[test]
    fn test_sign_v4_deterministic() {
        let policy = PostPolicy::new()
            .expiration_rfc3339("2030-01-01T00:00:00Z")
            .condition(serde_json::json!({"bucket": "examplebucket"}));
        let ts: Timestamp = "2024-01-01T00:00:00Z".parse().unwrap();

        let signed = sign_post_object_v4(PostObjectSignV4Input {
            access_key_id: "AKID",
            access_key_secret: "SECRET",
            security_token: None,
            region: "cn-hangzhou",
            product: "oss",
            policy: &policy,
            timestamp: ts,
        })
        .unwrap();
        assert_eq!(signed.signature_version, "OSS4-HMAC-SHA256");
        assert_eq!(signed.date, "20240101T000000Z");
        assert_eq!(signed.credential, "AKID/20240101/cn-hangzhou/oss/aliyun_v4_request");
        // Same inputs should always yield the same signature.
        let again = sign_post_object_v4(PostObjectSignV4Input {
            access_key_id: "AKID",
            access_key_secret: "SECRET",
            security_token: None,
            region: "cn-hangzhou",
            product: "oss",
            policy: &policy,
            timestamp: ts,
        })
        .unwrap();
        assert_eq!(signed.signature, again.signature);
        // Signature is hex.
        assert!(signed.signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_sign_v1() {
        let policy = PostPolicy::new().expiration_rfc3339("2030-01-01T00:00:00Z");
        let signed = sign_post_object_v1(PostObjectSignV1Input {
            access_key_id: "AKID",
            access_key_secret: "SECRET",
            security_token: None,
            policy: &policy,
        })
        .unwrap();
        assert_eq!(signed.credential, "AKID");
        assert!(!signed.signature.is_empty());
        assert_eq!(signed.signature_version, ""); // V1 has no version marker
    }

    #[test]
    fn test_build_form_fields_v4() {
        let signed = PostObjectFormFields {
            policy: "POLICY".into(),
            signature_version: "OSS4-HMAC-SHA256".into(),
            credential: "AKID/20240101/cn-hangzhou/oss/aliyun_v4_request".into(),
            date: "20240101T000000Z".into(),
            signature: "abcd".into(),
            security_token: None,
        };
        let fields = build_form_fields(&signed, "user/eric/file.jpg", BTreeMap::new());
        assert_eq!(fields.get("key").unwrap(), "user/eric/file.jpg");
        assert_eq!(fields.get("policy").unwrap(), "POLICY");
        assert_eq!(fields.get("x-oss-signature").unwrap(), "abcd");
        assert!(!fields.contains_key("OSSAccessKeyId"));
    }

    #[test]
    fn test_build_form_fields_v1() {
        let signed = PostObjectFormFields {
            policy: "POLICY".into(),
            signature_version: String::new(),
            credential: "AKID".into(),
            date: String::new(),
            signature: "base64sig==".into(),
            security_token: None,
        };
        let fields = build_form_fields(&signed, "file.jpg", BTreeMap::new());
        assert_eq!(fields.get("OSSAccessKeyId").unwrap(), "AKID");
        assert_eq!(fields.get("Signature").unwrap(), "base64sig==");
        assert!(!fields.contains_key("x-oss-signature"));
    }
}
