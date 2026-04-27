//! RRSA (RAM Roles for Service Accounts) credentials provider.
//!
//! The provider reads an OIDC token from a local file and calls the Alibaba
//! Cloud STS `AssumeRoleWithOIDC` API to exchange it for temporary
//! credentials. It is most commonly used inside ACK (Alibaba Cloud Container
//! Service for Kubernetes) pods that have been configured with a
//! `ServiceAccount` bound to a RAM role via an OIDC identity provider.
//!
//! The expected environment variables, injected by ACK, are:
//!
//! * `ALIBABA_CLOUD_ROLE_ARN` – ARN of the RAM role to assume.
//! * `ALIBABA_CLOUD_OIDC_PROVIDER_ARN` – ARN of the OIDC identity provider.
//! * `ALIBABA_CLOUD_OIDC_TOKEN_FILE` – path to the OIDC JWT token file.
//!
//! See <https://help.aliyun.com/zh/ram/developer-reference/api-sts-2015-04-01-assumerolewithoidc>
//! for the full API reference.

use std::path::{Path, PathBuf};
use std::time::Duration;

use jiff::Timestamp;
use serde::Deserialize;
use tracing::debug;

use crate::credentials::{CachingCredentialsProvider, Credentials, CredentialsProvider};
use crate::{Error, Result};

const DEFAULT_STS_ENDPOINT: &str = "https://sts.aliyuncs.com";
const STS_API_VERSION: &str = "2015-04-01";
const DEFAULT_SESSION_DURATION: u32 = 3600;
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// Default refresh-skew passed to the internal `CachingCredentialsProvider`.
/// Matches the cache module's own default (5 minutes).
const DEFAULT_REFRESH_SKEW: Duration = Duration::from_secs(5 * 60);

fn default_role_session_name() -> String {
    let ts = Timestamp::now().as_second();
    format!("ossify-rrsa-session-{ts}")
}

/// Ensure the STS endpoint has an `https://` scheme.
///
/// ACK injects `ALIBABA_CLOUD_STS_ENDPOINT` as a bare hostname such as
/// `sts-vpc.ap-southeast-1.aliyuncs.com` (no scheme). Passing a scheme-less
/// string directly to `reqwest` causes `RelativeUrlWithoutBase`. This
/// function normalises the value so callers never have to worry about it.
fn normalize_sts_endpoint(endpoint: impl Into<String>) -> String {
    let endpoint = endpoint.into();
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        endpoint
    } else {
        format!("https://{endpoint}")
    }
}

/// Builder for [`RrsaCredentialsProvider`].
#[derive(Debug, Clone)]
pub struct RrsaCredentialsProviderBuilder {
    role_arn: Option<String>,
    oidc_provider_arn: Option<String>,
    oidc_token_file_path: Option<PathBuf>,
    role_session_name: Option<String>,
    policy: Option<String>,
    session_duration_seconds: u32,
    sts_endpoint: String,
    http_client: Option<reqwest::Client>,
    refresh_skew: Duration,
}

impl Default for RrsaCredentialsProviderBuilder {
    fn default() -> Self {
        Self {
            role_arn: None,
            oidc_provider_arn: None,
            oidc_token_file_path: None,
            role_session_name: None,
            policy: None,
            session_duration_seconds: DEFAULT_SESSION_DURATION,
            sts_endpoint: DEFAULT_STS_ENDPOINT.to_string(),
            http_client: None,
            refresh_skew: DEFAULT_REFRESH_SKEW,
        }
    }
}

impl RrsaCredentialsProviderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// ARN of the RAM role to assume, e.g. `acs:ram::123456789012:role/foo`.
    pub fn role_arn(mut self, arn: impl Into<String>) -> Self {
        self.role_arn = Some(arn.into());
        self
    }

    /// ARN of the OIDC identity provider registered in RAM.
    pub fn oidc_provider_arn(mut self, arn: impl Into<String>) -> Self {
        self.oidc_provider_arn = Some(arn.into());
        self
    }

    /// Path to the file containing the OIDC token (JWT). The file is re-read
    /// on every refresh.
    pub fn oidc_token_file_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.oidc_token_file_path = Some(path.into());
        self
    }

    /// Session name passed to `AssumeRoleWithOIDC`. Defaults to a timestamp
    /// based value if not set.
    pub fn role_session_name(mut self, name: impl Into<String>) -> Self {
        self.role_session_name = Some(name.into());
        self
    }

    /// Optional policy document that further restricts the STS token's
    /// permissions.
    pub fn policy(mut self, policy: impl Into<String>) -> Self {
        self.policy = Some(policy.into());
        self
    }

    /// Session duration in seconds. Must be between 900 and the role's
    /// `MaxSessionDuration`. Defaults to 3600.
    pub fn session_duration_seconds(mut self, seconds: u32) -> Self {
        self.session_duration_seconds = seconds;
        self
    }

    /// Override the STS endpoint (defaults to `https://sts.aliyuncs.com`).
    ///
    /// The endpoint may be given with or without an `https://` scheme prefix;
    /// a missing scheme is treated as `https://`.
    pub fn sts_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.sts_endpoint = normalize_sts_endpoint(endpoint);
        self
    }

    /// Reuse an existing `reqwest::Client` for STS calls.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// How long before expiration to proactively refresh credentials.
    ///
    /// Defaults to 5 minutes. Set this higher (e.g. `Duration::from_secs(600)`)
    /// if you want a larger safety margin between a token refresh and its use.
    pub fn refresh_skew(mut self, skew: Duration) -> Self {
        self.refresh_skew = skew;
        self
    }

    pub fn build(self) -> Result<RrsaCredentialsProvider> {
        let role_arn = self
            .role_arn
            .ok_or_else(|| Error::InvalidArgument("rrsa: role_arn is required".to_string()))?;
        let oidc_provider_arn = self
            .oidc_provider_arn
            .ok_or_else(|| Error::InvalidArgument("rrsa: oidc_provider_arn is required".to_string()))?;
        let oidc_token_file_path = self
            .oidc_token_file_path
            .ok_or_else(|| Error::InvalidArgument("rrsa: oidc_token_file_path is required".to_string()))?;

        let http_client = self.http_client.unwrap_or_else(|| {
            reqwest::Client::builder()
                .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
                .build()
                .expect("default reqwest client")
        });

        Ok(RrsaCredentialsProvider {
            inner: CachingCredentialsProvider::new(RrsaInner {
                role_arn,
                oidc_provider_arn,
                oidc_token_file_path,
                role_session_name: self.role_session_name.unwrap_or_else(default_role_session_name),
                policy: self.policy,
                session_duration_seconds: self.session_duration_seconds,
                sts_endpoint: self.sts_endpoint,
                http_client,
            })
            .with_refresh_skew(self.refresh_skew),
        })
    }
}

/// Credentials provider that implements the RRSA (OIDC Role ARN) flow.
///
/// Use [`RrsaCredentialsProvider::from_env`] for the typical case of reading
/// configuration from environment variables, or build one manually with
/// [`RrsaCredentialsProviderBuilder`].
#[derive(Debug)]
pub struct RrsaCredentialsProvider {
    inner: CachingCredentialsProvider<RrsaInner>,
}

impl RrsaCredentialsProvider {
    pub fn builder() -> RrsaCredentialsProviderBuilder {
        RrsaCredentialsProviderBuilder::new()
    }

    /// Attempt to build a provider from the standard ACK-injected environment
    /// variables. Returns `None` if any of the required variables are missing.
    pub fn from_env(http_client: reqwest::Client) -> Option<Self> {
        let role_arn = std::env::var("ALIBABA_CLOUD_ROLE_ARN")
            .ok()
            .filter(|s| !s.is_empty())?;
        let oidc_provider_arn = std::env::var("ALIBABA_CLOUD_OIDC_PROVIDER_ARN")
            .ok()
            .filter(|s| !s.is_empty())?;
        let oidc_token_file = std::env::var("ALIBABA_CLOUD_OIDC_TOKEN_FILE")
            .ok()
            .filter(|s| !s.is_empty())?;

        let mut builder = Self::builder()
            .role_arn(role_arn)
            .oidc_provider_arn(oidc_provider_arn)
            .oidc_token_file_path(oidc_token_file)
            .http_client(http_client);

        if let Ok(name) = std::env::var("ALIBABA_CLOUD_ROLE_SESSION_NAME")
            && !name.is_empty()
        {
            builder = builder.role_session_name(name);
        }

        if let Ok(endpoint) = std::env::var("ALIBABA_CLOUD_STS_ENDPOINT")
            && !endpoint.is_empty()
        {
            builder = builder.sts_endpoint(normalize_sts_endpoint(endpoint));
        }

        builder.build().ok()
    }
}

impl CredentialsProvider for RrsaCredentialsProvider {
    async fn get_credentials(&self) -> Result<Credentials> {
        self.inner.get_credentials().await
    }
}

/// The inner, non-caching RRSA provider. Caching is layered on top via
/// [`CachingCredentialsProvider`].
#[derive(Debug)]
struct RrsaInner {
    role_arn: String,
    oidc_provider_arn: String,
    oidc_token_file_path: PathBuf,
    role_session_name: String,
    policy: Option<String>,
    session_duration_seconds: u32,
    sts_endpoint: String,
    http_client: reqwest::Client,
}

impl CredentialsProvider for RrsaInner {
    async fn get_credentials(&self) -> Result<Credentials> {
        let token = read_token_file(&self.oidc_token_file_path).await?;
        assume_role_with_oidc(self, &token).await
    }
}

async fn read_token_file(path: &Path) -> Result<String> {
    let bytes = tokio::fs::read(path)
        .await
        .map_err(|e| Error::Other(format!("rrsa: failed to read OIDC token file {}: {e}", path.display())))?;
    let token = String::from_utf8(bytes)
        .map_err(|_| Error::Other("rrsa: OIDC token file is not valid UTF-8".to_string()))?
        .trim()
        .to_string();
    if token.is_empty() {
        return Err(Error::Other("rrsa: OIDC token file is empty".to_string()));
    }
    Ok(token)
}

async fn assume_role_with_oidc(inner: &RrsaInner, oidc_token: &str) -> Result<Credentials> {
    // AssumeRoleWithOIDC is an anonymous (unsigned) RPC-style API: parameters
    // are passed as application/x-www-form-urlencoded body; response is JSON.
    let body = {
        let now = Timestamp::now();
        // STS RPC-style APIs require a Timestamp in ISO 8601 UTC format,
        // e.g. "2026-04-29T03:10:08Z".  Without it the server returns
        // MissingTimestamp (400).
        let timestamp = now.strftime("%Y-%m-%dT%H:%M:%SZ").to_string();

        let mut form = url::form_urlencoded::Serializer::new(String::new());
        form.append_pair("Action", "AssumeRoleWithOIDC");
        form.append_pair("Version", STS_API_VERSION);
        form.append_pair("Format", "JSON");
        form.append_pair("Timestamp", &timestamp);
        form.append_pair("RoleArn", &inner.role_arn);
        form.append_pair("OIDCProviderArn", &inner.oidc_provider_arn);
        form.append_pair("OIDCToken", oidc_token);
        form.append_pair("RoleSessionName", &inner.role_session_name);
        form.append_pair("DurationSeconds", &inner.session_duration_seconds.to_string());
        if let Some(policy) = &inner.policy {
            form.append_pair("Policy", policy);
        }
        form.finish()
    };

    debug!(
        target: "ossify::credentials::rrsa",
        role_arn = %inner.role_arn,
        oidc_provider_arn = %inner.oidc_provider_arn,
        "calling AssumeRoleWithOIDC",
    );

    let response = inner
        .http_client
        .post(&inner.sts_endpoint)
        .header(http::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(http::header::ACCEPT, "application/json")
        .body(body)
        .send()
        .await?;

    let status = response.status();
    let bytes = response.bytes().await?;
    let text = String::from_utf8_lossy(&bytes);
    if !status.is_success() {
        return Err(Error::Other(format!(
            "rrsa: AssumeRoleWithOIDC failed with status {status}: {text}"
        )));
    }

    let parsed: AssumeRoleWithOidcResponse = serde_json::from_slice(&bytes).map_err(|e| {
        Error::Other(format!("rrsa: failed to parse AssumeRoleWithOIDC response: {e}, body: {text}"))
    })?;

    let creds = parsed.credentials.ok_or_else(|| {
        Error::Other(format!("rrsa: AssumeRoleWithOIDC response missing Credentials, body: {text}"))
    })?;

    let expiration = parse_iso8601_utc(&creds.expiration)
        .map_err(|e| Error::Other(format!("rrsa: failed to parse expiration `{}`: {e}", creds.expiration)))?;

    Ok(Credentials::with_sts(
        creds.access_key_id,
        creds.access_key_secret,
        creds.security_token,
        Some(expiration),
    ))
}

fn parse_iso8601_utc(s: &str) -> std::result::Result<Timestamp, jiff::Error> {
    // STS returns times like `2021-10-20T04:27:09Z`. `jiff::Timestamp` parses
    // RFC 3339 directly via `FromStr`.
    s.parse::<Timestamp>()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AssumeRoleWithOidcResponse {
    #[serde(default)]
    credentials: Option<StsCredentials>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StsCredentials {
    access_key_id: String,
    access_key_secret: String,
    security_token: String,
    expiration: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iso8601_utc() {
        let ts = parse_iso8601_utc("2021-10-20T04:27:09Z").unwrap();
        assert_eq!(ts.as_second(), 1_634_704_029);
    }

    #[test]
    fn test_timestamp_format() {
        // Verify the Timestamp format string produces a valid ISO 8601 UTC
        // string that STS accepts (no fractional seconds, ends with "Z").
        let ts = parse_iso8601_utc("2021-10-20T04:27:09Z").unwrap();
        let formatted = ts.strftime("%Y-%m-%dT%H:%M:%SZ").to_string();
        assert_eq!(formatted, "2021-10-20T04:27:09Z");
    }

    #[test]
    fn test_builder_requires_fields() {
        let err = RrsaCredentialsProviderBuilder::new().build().unwrap_err();
        assert!(matches!(err, Error::InvalidArgument(_)));
    }

    #[test]
    fn test_builder_refresh_skew_is_stored() {
        // Build a complete provider and verify the refresh_skew was threaded
        // through. We check the builder field directly before build().
        let builder = RrsaCredentialsProviderBuilder::new().refresh_skew(Duration::from_secs(120));
        assert_eq!(builder.refresh_skew, Duration::from_secs(120));
    }

    #[test]
    fn test_normalize_sts_endpoint_bare_host() {
        // ACK injects the STS endpoint without a scheme, e.g.:
        //   ALIBABA_CLOUD_STS_ENDPOINT=sts-vpc.ap-southeast-1.aliyuncs.com
        assert_eq!(
            normalize_sts_endpoint("sts-vpc.ap-southeast-1.aliyuncs.com"),
            "https://sts-vpc.ap-southeast-1.aliyuncs.com",
        );
    }

    #[test]
    fn test_normalize_sts_endpoint_with_https() {
        // Already has https:// – must be left as-is.
        assert_eq!(normalize_sts_endpoint("https://sts.aliyuncs.com"), "https://sts.aliyuncs.com",);
    }

    #[test]
    fn test_normalize_sts_endpoint_with_http() {
        // Explicit http:// – left as-is (unusual but valid for local testing).
        assert_eq!(
            normalize_sts_endpoint("http://sts.example.internal"),
            "http://sts.example.internal",
        );
    }

    #[test]
    fn test_builder_sts_endpoint_normalizes() {
        // Verify the builder method also normalizes.
        let builder = RrsaCredentialsProviderBuilder::new().sts_endpoint("sts-vpc.cn-shanghai.aliyuncs.com");
        assert_eq!(builder.sts_endpoint, "https://sts-vpc.cn-shanghai.aliyuncs.com");
    }

    #[test]
    fn test_parse_assume_role_response() {
        // Sample response copied from the Alibaba Cloud STS documentation.
        let body = r#"{
            "RequestId": "3D57EAD2-8723-1F26-B69C-F8707D8B565D",
            "Credentials": {
                "SecurityToken": "CAIShwJ1q6Ft5B2yfSjIr5bSEsj4g7BihPWGWHz****",
                "Expiration": "2021-10-20T04:27:09Z",
                "AccessKeySecret": "CVwjCkNzTMupZ8NbTCxCBRq3K16jtcWFTJAyBEv2****",
                "AccessKeyId": "STS.NUgYrLnoC37mZZCNnAbez****"
            }
        }"#;

        let parsed: AssumeRoleWithOidcResponse = serde_json::from_str(body).unwrap();
        let creds = parsed.credentials.unwrap();
        assert_eq!(creds.access_key_id, "STS.NUgYrLnoC37mZZCNnAbez****");
        assert_eq!(creds.expiration, "2021-10-20T04:27:09Z");
    }
}
