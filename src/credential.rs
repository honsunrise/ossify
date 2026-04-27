use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Write;

use anyhow::{Context, Result};
use http::header::{AUTHORIZATION, DATE};
use http::{HeaderMap, HeaderValue};
use jiff::Timestamp;
use jiff::fmt::rfc2822;
use serde::Serialize;

use crate::credentials::Credentials;
use crate::utils::escape_path;
use crate::{QueryAuthOptions, ser};

const UNSIGNED_PAYLOAD: &str = "UNSIGNED-PAYLOAD";
const SIGNATURE_VERSION: &str = "OSS4-HMAC-SHA256";

pub(crate) struct SignContext<'a, Q>
where
    Q: Serialize,
{
    pub region: Cow<'a, str>,
    pub product: Cow<'a, str>,
    pub bucket: Option<Cow<'a, str>>,
    pub key: Option<Cow<'a, str>>,
    pub query: Option<&'a Q>,
    pub additional_headers: HashSet<Cow<'a, str>>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct WithCredentialQuery<'a, Q>
where
    Q: Serialize,
{
    x_oss_client: Cow<'a, str>,
    x_oss_date: Cow<'a, str>,
    x_oss_signature_version: Cow<'a, str>,
    x_oss_credential: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x_oss_security_token: Option<Cow<'a, str>>,
    #[serde(flatten)]
    query: Option<Q>,
    #[serde(flatten)]
    query_auth_options: QueryAuthOptions,
}

pub(crate) fn auth_to<Q>(
    credentials: &Credentials,
    request: &mut reqwest::Request,
    SignContext {
        region,
        product,
        bucket,
        key,
        additional_headers,
        query,
    }: SignContext<'_, Q>,
    query_auth_options: Option<QueryAuthOptions>,
) -> Result<()>
where
    Q: Serialize,
{
    static RFC2822_PRINTER: rfc2822::DateTimePrinter = rfc2822::DateTimePrinter::new();

    let is_query_auth = query_auth_options.is_some();

    // Prepare x-sdk-client
    let version = env!("CARGO_PKG_VERSION");
    let x_sdk_client = format!("ossify/{version}");

    // Prepare x-oss-date and date
    let datetime = Timestamp::now();
    let datetime_iso8601_str = datetime.strftime("%Y%m%dT%H%M%SZ").to_string();
    let datetime_rfc2822_str = RFC2822_PRINTER.timestamp_to_string(&datetime)?;

    // Prepare scope
    let date_iso8601_str = &datetime_iso8601_str[..8];
    let scope = build_scope(date_iso8601_str, &region, &product);

    // Canonical sign path
    let sign_path = build_sign_path(bucket.as_deref(), key.as_deref());
    let canonical_sign_path = escape_path(&sign_path);

    // Canonical query
    let mut canonical_query: Cow<'_, str> = Cow::Borrowed("");
    if let Some(query_auth_options) = query_auth_options {
        let with_credential = WithCredentialQuery {
            x_oss_credential: Cow::Owned(format!("{}/{scope}", credentials.access_key_id)),
            x_oss_client: Cow::Borrowed(&x_sdk_client),
            x_oss_date: Cow::Borrowed(&datetime_iso8601_str),
            x_oss_signature_version: Cow::Borrowed(SIGNATURE_VERSION),
            x_oss_security_token: credentials.security_token.as_deref().map(Cow::Borrowed),
            query_auth_options,
            query,
        };
        canonical_query = Cow::Owned(ser::to_string(&with_credential)?)
    } else if let Some(query) = query {
        canonical_query = Cow::Owned(ser::to_string(&query)?)
    }

    // Append headers
    let mut canonical_headers_str = Cow::Borrowed("");
    let mut canonical_additional_headers_str = Cow::Borrowed("");
    if !is_query_auth {
        let x_oss_content_sha256 = HeaderValue::from_static(UNSIGNED_PAYLOAD);
        let x_sdk_client = HeaderValue::from_str(&x_sdk_client).context("parse x-sdk-client")?;
        let x_oss_date = HeaderValue::from_str(&datetime_iso8601_str).expect("invalid x-oss-date");
        let date_rfc2822 = HeaderValue::from_str(&datetime_rfc2822_str).expect("invalid date");

        let headers = request.headers_mut();
        headers.append("x-sdk-client", x_sdk_client);
        headers.append("x-oss-date", x_oss_date);
        headers.append(DATE, date_rfc2822);
        headers.append("x-oss-content-sha256", x_oss_content_sha256);

        // Append security token header if present
        if let Some(token) = &credentials.security_token {
            headers.insert("x-oss-security-token", HeaderValue::from_str(token)?);
        }

        // Canonical headers
        canonical_headers_str = Cow::Owned(canonical_headers(headers, &additional_headers)?);
        canonical_additional_headers_str = Cow::Owned(
            additional_headers
                .iter()
                .map(|h| h.to_lowercase())
                .collect::<Vec<_>>()
                .join(";"),
        );
    };

    // Prepare Authorization
    let method = request.method();
    let canonical_request = format!(
        "{}\n{canonical_sign_path}\n{canonical_query}\n{canonical_headers_str}\n{canonical_additional_headers_str}\n{UNSIGNED_PAYLOAD}",
        method.as_str(),
    );

    // Prepare string to sign
    let string_to_sign = format!(
        "{SIGNATURE_VERSION}\n{datetime_iso8601_str}\n{scope}\n{}",
        sha256_hex(&canonical_request)
    );

    let signature = hex::encode(calculate_signature(
        &credentials.access_key_secret,
        date_iso8601_str,
        &region,
        &product,
        &string_to_sign,
    )?);

    if is_query_auth {
        canonical_query = Cow::Owned(format!("{canonical_query}&x-oss-signature={signature}"));
    } else {
        let mut credential_header =
            format!("{SIGNATURE_VERSION} Credential={}/{scope}", credentials.access_key_id);
        if !canonical_additional_headers_str.is_empty() {
            write!(&mut credential_header, ",AdditionalHeaders={canonical_additional_headers_str}")?;
        }
        write!(&mut credential_header, ",Signature={signature}")?;
        let authorization = HeaderValue::from_str(&credential_header).expect("invalid Authorization");
        let headers = request.headers_mut();
        headers.append(AUTHORIZATION, authorization);
    }

    if !canonical_query.is_empty() {
        request.url_mut().set_query(Some(&canonical_query));
    }

    Ok(())
}

#[inline]
fn build_sign_path(bucket: Option<&str>, key: Option<&str>) -> String {
    match (bucket, key) {
        (Some(bucket), Some(key)) => format!("/{bucket}/{key}"),
        (Some(bucket), None) => format!("/{bucket}/"),
        (None, Some(key)) => format!("/{key}"),
        (None, None) => "/".to_string(),
    }
}

#[inline]
fn build_scope(date_iso8601_str: &str, region: &str, product: &str) -> String {
    format!("{date_iso8601_str}/{region}/{product}/aliyun_v4_request")
}

#[inline]
pub(crate) fn sha256_hex(message: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(message);
    hex::encode(hasher.finalize())
}

#[inline]
pub(crate) fn hmac256(key: &[u8], message: &str) -> Result<Vec<u8>> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(key)?;
    mac.update(message.as_bytes());
    let signature = mac.finalize();
    Ok(signature.into_bytes().to_vec())
}

fn calculate_signature(
    access_key_secret: &str,
    date_iso8601_str: &str,
    region: &str,
    product: &str,
    string_to_sign: &str,
) -> Result<Vec<u8>> {
    let key_string = format!("aliyun_v4{access_key_secret}");
    let date_key = hmac256(key_string.as_bytes(), date_iso8601_str)?;
    let date_region_key = hmac256(&date_key, region)?;
    let date_region_service_key = hmac256(&date_region_key, product)?;
    let signing_key = hmac256(&date_region_service_key, "aliyun_v4_request")?;
    let signature = hmac256(&signing_key, string_to_sign)?;
    Ok(signature)
}

fn canonical_headers(input: &HeaderMap, additional_headers: &HashSet<Cow<'_, str>>) -> Result<String> {
    use std::fmt::Write;

    let mut headers = Vec::with_capacity(input.len());
    let filter_input = input.iter().filter(|(k, _)| {
        k.as_str().starts_with("x-oss-")
            || k.as_str() == "content-md5"
            || k.as_str() == "content-type"
            || additional_headers.contains(k.as_str())
    });
    for (k, v) in filter_input {
        headers.push((k.as_str().to_lowercase(), v.to_str()?.trim()));
    }
    headers.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

    Ok(headers.into_iter().fold(String::new(), |mut output, (k, v)| {
        let _ = writeln!(output, "{k}:{v}");
        output
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_sign_path_bucket_and_key() {
        assert_eq!(build_sign_path(Some("my-bucket"), Some("my-key")), "/my-bucket/my-key");
    }

    #[test]
    fn test_build_sign_path_bucket_only() {
        assert_eq!(build_sign_path(Some("my-bucket"), None), "/my-bucket/");
    }

    #[test]
    fn test_build_sign_path_key_only() {
        assert_eq!(build_sign_path(None, Some("my-key")), "/my-key");
    }

    #[test]
    fn test_build_sign_path_none() {
        assert_eq!(build_sign_path(None, None), "/");
    }

    #[test]
    fn test_build_sign_path_with_nested_key() {
        assert_eq!(
            build_sign_path(Some("bucket"), Some("path/to/object.txt")),
            "/bucket/path/to/object.txt"
        );
    }

    #[test]
    fn test_build_scope() {
        assert_eq!(
            build_scope("20240101", "cn-hangzhou", "oss"),
            "20240101/cn-hangzhou/oss/aliyun_v4_request"
        );
    }

    #[test]
    fn test_sha256_hex_empty() {
        // SHA-256 of empty string
        assert_eq!(
            sha256_hex(""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_sha256_hex_known_value() {
        // SHA-256 of "hello"
        assert_eq!(
            sha256_hex("hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_hmac256() {
        let result = hmac256(b"key", "message").unwrap();
        assert_eq!(result.len(), 32); // HMAC-SHA256 always produces 32 bytes
        assert_eq!(
            hex::encode(&result),
            "6e9ef29b75fffc5b7abae527d58fdadb2fe42e7219011976917343065f58ed4a"
        );
    }

    #[test]
    fn test_calculate_signature_deterministic() {
        let sig1 = calculate_signature("secret", "20240101", "cn-hangzhou", "oss", "test-string").unwrap();
        let sig2 = calculate_signature("secret", "20240101", "cn-hangzhou", "oss", "test-string").unwrap();
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_calculate_signature_different_secrets() {
        let sig1 = calculate_signature("secret1", "20240101", "cn-hangzhou", "oss", "test").unwrap();
        let sig2 = calculate_signature("secret2", "20240101", "cn-hangzhou", "oss", "test").unwrap();
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_calculate_signature_different_dates() {
        let sig1 = calculate_signature("secret", "20240101", "cn-hangzhou", "oss", "test").unwrap();
        let sig2 = calculate_signature("secret", "20240202", "cn-hangzhou", "oss", "test").unwrap();
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_calculate_signature_different_regions() {
        let sig1 = calculate_signature("secret", "20240101", "cn-hangzhou", "oss", "test").unwrap();
        let sig2 = calculate_signature("secret", "20240101", "cn-beijing", "oss", "test").unwrap();
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_canonical_headers_filters_x_oss_prefix() {
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-date", HeaderValue::from_static("20240101T000000Z"));
        headers.insert("x-oss-content-sha256", HeaderValue::from_static("UNSIGNED-PAYLOAD"));
        headers.insert("host", HeaderValue::from_static("example.com"));

        let additional = HashSet::new();
        let result = canonical_headers(&headers, &additional).unwrap();

        assert!(result.contains("x-oss-content-sha256:UNSIGNED-PAYLOAD\n"));
        assert!(result.contains("x-oss-date:20240101T000000Z\n"));
        assert!(!result.contains("host"));
    }

    #[test]
    fn test_canonical_headers_includes_content_type() {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("accept", HeaderValue::from_static("*/*"));

        let additional = HashSet::new();
        let result = canonical_headers(&headers, &additional).unwrap();

        assert!(result.contains("content-type:application/json\n"));
        assert!(!result.contains("accept"));
    }

    #[test]
    fn test_canonical_headers_includes_content_md5() {
        let mut headers = HeaderMap::new();
        headers.insert("content-md5", HeaderValue::from_static("abc123"));

        let additional = HashSet::new();
        let result = canonical_headers(&headers, &additional).unwrap();

        assert!(result.contains("content-md5:abc123\n"));
    }

    #[test]
    fn test_canonical_headers_includes_additional_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("host", HeaderValue::from_static("example.com"));
        headers.insert("x-custom", HeaderValue::from_static("value"));

        let mut additional = HashSet::new();
        additional.insert(Cow::Borrowed("host"));

        let result = canonical_headers(&headers, &additional).unwrap();

        assert!(result.contains("host:example.com\n"));
        assert!(!result.contains("x-custom"));
    }

    #[test]
    fn test_canonical_headers_sorted() {
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-z", HeaderValue::from_static("z"));
        headers.insert("x-oss-a", HeaderValue::from_static("a"));
        headers.insert("x-oss-m", HeaderValue::from_static("m"));

        let additional = HashSet::new();
        let result = canonical_headers(&headers, &additional).unwrap();

        let lines: Vec<&str> = result.trim_end().split('\n').collect();
        assert_eq!(lines, vec!["x-oss-a:a", "x-oss-m:m", "x-oss-z:z"]);
    }

    #[test]
    fn test_canonical_headers_trims_values() {
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-test", HeaderValue::from_static("  value  "));

        let additional = HashSet::new();
        let result = canonical_headers(&headers, &additional).unwrap();

        assert!(result.contains("x-oss-test:value\n"));
    }

    #[test]
    fn test_canonical_headers_empty() {
        let headers = HeaderMap::new();
        let additional = HashSet::new();
        let result = canonical_headers(&headers, &additional).unwrap();
        assert_eq!(result, "");
    }

    fn build_test_request(url: &str) -> reqwest::Request {
        let _ = rustls::crypto::ring::default_provider().install_default();
        let client = reqwest::Client::new();
        client.get(url).build().unwrap()
    }

    #[test]
    fn test_auth_to_adds_authorization_header() {
        let credentials = Credentials {
            access_key_id: "test-ak-id".to_string(),
            access_key_secret: "test-ak-secret".to_string(),
            security_token: None,
            expiration: None,
        };

        let mut request = build_test_request("https://example.oss-cn-hangzhou.aliyuncs.com/");

        let sign_context: SignContext<'_, ()> = SignContext {
            region: Cow::Borrowed("cn-hangzhou"),
            product: Cow::Borrowed("oss"),
            bucket: Some(Cow::Borrowed("test-bucket")),
            key: Some(Cow::Borrowed("test-key")),
            query: None,
            additional_headers: HashSet::new(),
        };

        auth_to(&credentials, &mut request, sign_context, None).unwrap();

        let headers = request.headers();
        assert!(headers.contains_key(AUTHORIZATION));
        let auth_value = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
        assert!(auth_value.starts_with("OSS4-HMAC-SHA256 Credential=test-ak-id/"));
        assert!(auth_value.contains(",Signature="));

        assert!(headers.contains_key("x-oss-date"));
        assert!(headers.contains_key("x-oss-content-sha256"));
        assert!(headers.contains_key("x-sdk-client"));
        assert!(headers.contains_key(DATE));
    }

    #[test]
    fn test_auth_to_with_security_token() {
        let credentials = Credentials {
            access_key_id: "test-ak-id".to_string(),
            access_key_secret: "test-ak-secret".to_string(),
            security_token: Some("test-security-token".to_string()),
            expiration: None,
        };

        let mut request = build_test_request("https://example.oss-cn-hangzhou.aliyuncs.com/");

        let sign_context: SignContext<'_, ()> = SignContext {
            region: Cow::Borrowed("cn-hangzhou"),
            product: Cow::Borrowed("oss"),
            bucket: Some(Cow::Borrowed("test-bucket")),
            key: None,
            query: None,
            additional_headers: HashSet::new(),
        };

        auth_to(&credentials, &mut request, sign_context, None).unwrap();

        let headers = request.headers();
        assert_eq!(
            headers.get("x-oss-security-token").unwrap().to_str().unwrap(),
            "test-security-token"
        );
    }

    #[test]
    fn test_auth_to_query_auth_sets_query_string() {
        let credentials = Credentials {
            access_key_id: "test-ak-id".to_string(),
            access_key_secret: "test-ak-secret".to_string(),
            security_token: None,
            expiration: None,
        };

        let mut request = build_test_request("https://example.oss-cn-hangzhou.aliyuncs.com/");

        let sign_context: SignContext<'_, ()> = SignContext {
            region: Cow::Borrowed("cn-hangzhou"),
            product: Cow::Borrowed("oss"),
            bucket: Some(Cow::Borrowed("test-bucket")),
            key: Some(Cow::Borrowed("test-key")),
            query: None,
            additional_headers: HashSet::new(),
        };

        let query_auth_options = QueryAuthOptions::builder().x_oss_expires(3600).build();
        auth_to(&credentials, &mut request, sign_context, Some(query_auth_options)).unwrap();

        // In query auth mode, no Authorization header should be set
        assert!(!request.headers().contains_key(AUTHORIZATION));
        // Query string should contain the signature
        let query = request.url().query().unwrap();
        assert!(query.contains("x-oss-signature="));
        assert!(query.contains("x-oss-credential="));
        assert!(query.contains("x-oss-signature-version=OSS4-HMAC-SHA256"));
        // No STS token was provided so it must not appear in the query string
        assert!(!query.contains("x-oss-security-token"));
    }

    #[test]
    fn test_auth_to_query_auth_with_security_token() {
        // When STS credentials are used, the security token must appear in
        // the presigned URL query string so the server can look up the
        // temporary key pair.
        let credentials = Credentials {
            access_key_id: "STS.test-sts-ak".to_string(),
            access_key_secret: "test-sts-secret".to_string(),
            security_token: Some("test-security-token".to_string()),
            expiration: None,
        };

        let mut request = build_test_request("https://example.oss-cn-hangzhou.aliyuncs.com/");

        let sign_context: SignContext<'_, ()> = SignContext {
            region: Cow::Borrowed("cn-hangzhou"),
            product: Cow::Borrowed("oss"),
            bucket: Some(Cow::Borrowed("test-bucket")),
            key: Some(Cow::Borrowed("test-key")),
            query: None,
            additional_headers: HashSet::new(),
        };

        let query_auth_options = QueryAuthOptions::builder().x_oss_expires(3600).build();
        auth_to(&credentials, &mut request, sign_context, Some(query_auth_options)).unwrap();

        let query = request.url().query().unwrap();
        // STS token must be present in the query string
        assert!(
            query.contains("x-oss-security-token=test-security-token"),
            "query missing x-oss-security-token: {query}"
        );
        // Signature fields must still be present
        assert!(query.contains("x-oss-signature="));
        assert!(query.contains("x-oss-credential=STS.test-sts-ak"));
    }
}
