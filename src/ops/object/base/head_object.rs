use std::borrow::Cow;
use std::future::Future;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use http::{HeaderMap, Method, header};
use serde::{Deserialize, Deserializer, Serialize};

use super::StorageClass;
use crate::body::EmptyBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::{Client, Ops, Request};

/// OSS object type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all_fields = "lowercase")]
pub enum ObjectType {
    /// Object uploaded via PutObject
    Normal,
    /// Object uploaded via AppendObject
    Appendable,
    /// Object uploaded via MultipartUpload
    Multipart,
}

/// Server-side encryption type
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all_fields = "lowercase")]
pub enum ServerSideEncryption {
    /// AES256 encryption
    AES256,
    /// KMS encryption
    KMS,
    /// SM4 encryption
    SM4,
}

/// Restore status information
#[derive(Debug, Clone, Deserialize)]
pub struct RestoreInfo {
    /// Whether a restore request is ongoing
    pub ongoing_request: bool,
    /// restoreExpiration time
    pub expiry_date: Option<String>,
}

impl FromStr for RestoreInfo {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let ongoing_request_regex = regex::Regex::new(r#"ongoing-request="([^"]+)""#).unwrap();
        let expiry_date_regex = regex::Regex::new(r#"expiry-date="([^"]+)""#).unwrap();

        // Parse restore header information, format: ongoing-request="true", expiry-date="Sun, 16 Apr 2017 08:12:33 GMT"
        let mut ongoing_request = false;
        let mut expiry_date = None;

        for part in s.split(',') {
            let part = part.trim();
            if let Some(captures) = ongoing_request_regex.captures(part) {
                ongoing_request = captures.get(1).unwrap().as_str() == "true";
            } else if let Some(captures) = expiry_date_regex.captures(part) {
                expiry_date = Some(captures.get(1).unwrap().as_str().to_string());
            }
        }

        Ok(RestoreInfo {
            ongoing_request,
            expiry_date,
        })
    }
}

fn deserialize_content_length<'de, D>(deserializer: D) -> std::result::Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

fn deserialize_datetime<'de, D>(deserializer: D) -> std::result::Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc2822(&s).map_err(serde::de::Error::custom)
}

/// HeadObject response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HeadObjectResponse {
    /// File size
    #[serde(deserialize_with = "deserialize_content_length")]
    pub content_length: u64,
    /// Content type
    pub content_type: String,
    /// Creation time
    #[serde(deserialize_with = "deserialize_datetime")]
    pub date: DateTime<FixedOffset>,
    /// Last modified time
    #[serde(deserialize_with = "deserialize_datetime")]
    pub last_modified: DateTime<FixedOffset>,
    /// ETag
    #[serde(rename = "etag")]
    pub etag: Option<String>,
    /// Version ID
    #[serde(rename = "x-oss-versionId")]
    pub version_id: Option<String>,
    /// Object type
    #[serde(rename = "x-oss-object-type")]
    pub object_type: Option<ObjectType>,
    /// Storage class
    #[serde(rename = "x-oss-storage-class")]
    pub storage_class: Option<StorageClass>,
    /// Server-side encryption
    #[serde(rename = "x-oss-server-side-encryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// Server-side encryption key ID
    #[serde(rename = "x-oss-server-side-encryption-key-id")]
    pub server_side_encryption_key_id: Option<String>,
    /// Next append position
    #[serde(rename = "x-oss-next-append-position")]
    pub next_append_position: Option<u64>,
    /// CRC64 value
    #[serde(rename = "x-oss-hash-crc64ecma")]
    pub hash_crc64ecma: Option<String>,
    /// Tag count
    #[serde(rename = "x-oss-tagging-count")]
    pub tagging_count: Option<u32>,
    /// Expiration time
    #[serde(rename = "x-oss-expiration")]
    pub expiration: Option<String>,
    /// Restore information
    #[serde(rename = "x-oss-restore")]
    pub restore: Option<RestoreInfo>,
    /// Source file
    #[serde(rename = "x-oss-meta-source")]
    pub source: Option<String>,
}

/// HeadObjectRequest parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct HeadObjectParams {
    /// Version ID
    pub version_id: Option<String>,
}

pub struct HeadObjectOptions {
    pub if_modified_since: Option<String>,
    pub if_unmodified_since: Option<String>,
    pub if_match: Option<String>,
    pub if_none_match: Option<String>,
}

/// HeadObjectRequest builder
#[derive(Debug, Clone)]
pub struct HeadObjectRequestBuilder {
    /// Return 200 OK and Object Meta if the time in the parameter is earlier than the actual modification time; otherwise return 304 Not Modified
    pub if_modified_since: Option<String>,
    /// Return 200 OK and Object Meta if the time in the parameter is equal to or later than the actual modification time; otherwise return 412 Precondition Failed
    pub if_unmodified_since: Option<String>,
    /// Return 200 OK and Object Meta if the expected ETag matches the Object's ETag; otherwise return 412 precondition failed
    pub if_match: Option<String>,
    /// Return 200 OK and Object Meta if the expected ETag value does not match the Object's ETag; otherwise return 304 Not Modified
    pub if_none_match: Option<String>,
}

impl HeadObjectRequestBuilder {
    pub fn new() -> Self {
        Self {
            if_modified_since: None,
            if_unmodified_since: None,
            if_match: None,
            if_none_match: None,
        }
    }

    /// Set the If-Modified-Since header
    pub fn if_modified_since(mut self, time: impl Into<String>) -> Self {
        self.if_modified_since = Some(time.into());
        self
    }

    /// Set the If-Unmodified-Since header
    pub fn if_unmodified_since(mut self, time: impl Into<String>) -> Self {
        self.if_unmodified_since = Some(time.into());
        self
    }

    /// Set the If-Match header
    pub fn if_match(mut self, etag: impl Into<String>) -> Self {
        self.if_match = Some(etag.into());
        self
    }

    /// Set the If-None-Match header
    pub fn if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.if_none_match = Some(etag.into());
        self
    }

    pub fn build(self) -> HeadObjectOptions {
        HeadObjectOptions {
            if_modified_since: self.if_modified_since,
            if_unmodified_since: self.if_unmodified_since,
            if_match: self.if_match,
            if_none_match: self.if_none_match,
        }
    }
}

impl Default for HeadObjectRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// HeadObject operation
pub struct HeadObject {
    pub object_name: String,
    pub params: HeadObjectParams,
    pub options: Option<HeadObjectOptions>,
}

impl Ops for HeadObject {
    type Response = HeaderResponseProcessor<HeadObjectResponse>;
    type Body = EmptyBody;
    type Query = HeadObjectParams;

    fn method(&self) -> Method {
        Method::HEAD
    }

    fn key<'a>(&'a self) -> Option<Cow<'a, str>> {
        Some(Cow::Borrowed(&self.object_name))
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.params)
    }

    fn headers(&self) -> Result<Option<HeaderMap>> {
        let mut headers = HeaderMap::new();
        let Some(options) = &self.options else {
            return Ok(None);
        };

        if let Some(if_modified_since) = &options.if_modified_since {
            headers.insert(header::IF_MODIFIED_SINCE, if_modified_since.parse()?);
        }

        if let Some(if_unmodified_since) = &options.if_unmodified_since {
            headers.insert(header::IF_UNMODIFIED_SINCE, if_unmodified_since.parse()?);
        }

        if let Some(if_match) = &options.if_match {
            headers.insert(header::IF_MATCH, if_match.parse()?);
        }

        if let Some(if_none_match) = &options.if_none_match {
            headers.insert(header::IF_NONE_MATCH, if_none_match.parse()?);
        }

        Ok(Some(headers))
    }
}

/// HeadObject operations trait
pub trait HeadObjectOperations {
    /// Get metadata for an object (file)
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/headobject>
    fn head_object(
        &self,
        object_name: impl AsRef<str>,
        params: HeadObjectParams,
        options: Option<HeadObjectOptions>,
    ) -> impl Future<Output = Result<HeadObjectResponse>>;
}

impl HeadObjectOperations for Client {
    async fn head_object(
        &self,
        object_name: impl AsRef<str>,
        params: HeadObjectParams,
        options: Option<HeadObjectOptions>,
    ) -> Result<HeadObjectResponse> {
        let ops = HeadObject {
            object_name: object_name.as_ref().to_string(),
            params,
            options,
        };

        self.request(ops).await
    }
}
