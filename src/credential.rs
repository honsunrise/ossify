use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Write;

use anyhow::{Context, Result};
use chrono::Utc;
use http::header::{AUTHORIZATION, DATE};
use http::{HeaderMap, HeaderValue};
use serde::Serialize;

use crate::utils::escape_path;
use crate::{QueryAuthOptions, ser};

const UNSIGNED_PAYLOAD: &str = "UNSIGNED-PAYLOAD";
const SIGNATURE_VERSION: &str = "OSS4-HMAC-SHA256";

#[derive(Clone, Debug)]
pub(crate) struct Credential {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub security_token: Option<String>,
}

pub(crate) struct SignContext<'a, Q>
where
    Q: Serialize,
{
    pub region: Cow<'a, str>,
    pub product: Cow<'a, str>,
    pub bucket: Option<Cow<'a, str>>,
    pub key: Option<Cow<'a, str>>,
    pub query: Option<&'a Q>,
    pub additional_headers: HashSet<String>,
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
    #[serde(flatten)]
    query: Option<Q>,
    #[serde(flatten)]
    query_auth_options: QueryAuthOptions,
}

impl Credential {
    pub(crate) fn auth_to<Q>(
        &self,
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
        let is_query_auth = query_auth_options.is_some();

        // Prepare x-sdk-client
        let version = env!("CARGO_PKG_VERSION");
        let x_sdk_client = format!("ossify/{version}");

        // Prepare x-oss-date and date
        let datetime = Utc::now();
        let datetime_iso8601_str = datetime.format("%Y%m%dT%H%M%SZ").to_string();
        let datetime_rfc2822_str = datetime.to_rfc2822();

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
                x_oss_credential: Cow::Owned(format!("{}/{scope}", self.access_key_id)),
                x_oss_client: Cow::Borrowed(&x_sdk_client),
                x_oss_date: Cow::Borrowed(&datetime_iso8601_str),
                x_oss_signature_version: Cow::Borrowed(SIGNATURE_VERSION),
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
            if let Some(token) = &self.security_token {
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
            &self.access_key_secret,
            date_iso8601_str,
            &region,
            &product,
            &string_to_sign,
        )?);

        if is_query_auth {
            canonical_query = Cow::Owned(format!("{canonical_query}&x-oss-signature={signature}"));
        } else {
            let mut credential_header =
                format!("{SIGNATURE_VERSION} Credential={}/{scope}", self.access_key_id);
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

fn canonical_headers(input: &HeaderMap, additional_headers: &HashSet<String>) -> Result<String> {
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
