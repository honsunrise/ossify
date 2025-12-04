use std::future::Future;

use bytes::Bytes;
use http::{HeaderMap, Method, header};
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BinaryResponseProcessor;
use crate::{Client, Ops, Prepared, QueryAuthOptions, Request};

/// GetObject request parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetObjectParams {
    /// Version ID for retrieving a specific version of the object
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Response Cache-Control header
    #[serde(rename = "response-cache-control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_cache_control: Option<String>,
    /// Response Content-Disposition header
    #[serde(rename = "response-content-disposition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_disposition: Option<String>,
    /// Response Content-Encoding header
    #[serde(rename = "response-content-encoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_encoding: Option<String>,
    /// Response Content-Language header
    #[serde(rename = "response-content-language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_language: Option<String>,
    /// Response Content-Type header
    #[serde(rename = "response-content-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_type: Option<String>,
    /// Response Expires header
    #[serde(rename = "response-expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_expires: Option<String>,
}

impl GetObjectParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Version ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// Set the response Cache-Control header
    pub fn response_cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.response_cache_control = Some(cache_control.into());
        self
    }

    /// Set the response Content-Disposition header
    pub fn response_content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.response_content_disposition = Some(content_disposition.into());
        self
    }

    /// Set the response Content-Encoding header
    pub fn response_content_encoding(mut self, content_encoding: impl Into<String>) -> Self {
        self.response_content_encoding = Some(content_encoding.into());
        self
    }

    /// Set the response Content-Language header
    pub fn response_content_language(mut self, content_language: impl Into<String>) -> Self {
        self.response_content_language = Some(content_language.into());
        self
    }

    /// Set the response Content-Type header
    pub fn response_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.response_content_type = Some(content_type.into());
        self
    }

    /// Set the response Expires header
    pub fn response_expires(mut self, expires: impl Into<String>) -> Self {
        self.response_expires = Some(expires.into());
        self
    }
}

/// GetObject request options (primarily set through HTTP headers)
#[derive(Debug, Clone, Default)]
pub struct GetObjectOptions {
    /// Specify the file transfer range (e.g., "bytes=0-1023")
    pub range: Option<String>,
    /// Return the object if the specified time is earlier than the actual modification time
    pub if_modified_since: Option<String>,
    /// Return the object if the specified time is equal to or later than the actual modification time
    pub if_unmodified_since: Option<String>,
    /// Return the object if the provided ETag matches the object's ETag
    pub if_match: Option<String>,
    /// Return the object if the provided ETag does not match the object's ETag
    pub if_none_match: Option<String>,
    /// Accepted encoding format
    pub accept_encoding: Option<String>,
}

impl GetObjectOptions {
    /// Set the Range header for segmented download
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
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

    /// Set the Accept-Encoding header
    pub fn accept_encoding(mut self, encoding: impl Into<String>) -> Self {
        self.accept_encoding = Some(encoding.into());
        self
    }
}

impl GetObjectOptions {
    fn into_headers(self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        // Set Range header
        if let Some(range) = self.range {
            headers.insert(header::RANGE, range.parse()?);
        }

        // Set conditional request headers
        if let Some(if_modified_since) = self.if_modified_since {
            headers.insert(header::IF_MODIFIED_SINCE, if_modified_since.parse()?);
        }

        if let Some(if_unmodified_since) = self.if_unmodified_since {
            headers.insert(header::IF_UNMODIFIED_SINCE, if_unmodified_since.parse()?);
        }

        if let Some(if_match) = self.if_match {
            headers.insert(header::IF_MATCH, if_match.parse()?);
        }

        if let Some(if_none_match) = self.if_none_match {
            headers.insert(header::IF_NONE_MATCH, if_none_match.parse()?);
        }

        // Set Accept-Encoding header
        if let Some(accept_encoding) = self.accept_encoding {
            headers.insert(header::ACCEPT_ENCODING, accept_encoding.parse()?);
        }

        Ok(headers)
    }
}

/// GetObject operation
pub struct GetObject {
    pub object_key: String,
    pub params: GetObjectParams,
    pub options: GetObjectOptions,
}

impl Ops for GetObject {
    type Response = BinaryResponseProcessor;
    type Body = NoneBody;
    type Query = GetObjectParams;

    fn prepare(self) -> Result<Prepared<GetObjectParams>> {
        Ok(Prepared {
            method: Method::GET,
            key: Some(self.object_key),
            query: Some(self.params),
            headers: Some(self.options.into_headers()?),
            ..Default::default()
        })
    }
}

/// GetObject operations trait
pub trait GetObjectOperations {
    /// Get an object (file)
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/getobject>
    fn get_object(
        &self,
        object_key: impl Into<String>,
        params: GetObjectParams,
        options: Option<GetObjectOptions>,
    ) -> impl Future<Output = Result<Bytes>>;

    fn presign_get_object(
        &self,
        object_key: impl Into<String>,
        public: bool,
        params: GetObjectParams,
        options: Option<GetObjectOptions>,
        query_auth_options: QueryAuthOptions,
    ) -> impl Future<Output = Result<String>>;
}

impl GetObjectOperations for Client {
    async fn get_object(
        &self,
        object_key: impl Into<String>,
        params: GetObjectParams,
        options: Option<GetObjectOptions>,
    ) -> Result<Bytes> {
        let ops = GetObject {
            object_key: object_key.into(),
            params,
            options: options.unwrap_or_default(),
        };

        self.request(ops).await
    }

    async fn presign_get_object(
        &self,
        object_key: impl Into<String>,
        public: bool,
        params: GetObjectParams,
        options: Option<GetObjectOptions>,
        query_auth_options: QueryAuthOptions,
    ) -> Result<String> {
        let ops = GetObject {
            object_key: object_key.into(),
            params,
            options: options.unwrap_or_default(),
        };
        self.presign(ops, public, Some(query_auth_options)).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// GetObjectRequest builder
#[derive(Debug, Clone, Default)]
pub struct GetObjectRequestBuilder {
    params: GetObjectParams,
    options: GetObjectOptions,
}

impl GetObjectRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Version ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.params.version_id = Some(version_id.into());
        self
    }

    /// Set Range header (for segmented download)
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.options.range = Some(range.into());
        self
    }

    /// Set Range header (by start and end positions)
    pub fn range_bytes(mut self, start: u64, end: Option<u64>) -> Self {
        let range = match end {
            Some(end) => format!("bytes={start}-{end}"),
            None => format!("bytes={start}-"),
        };
        self.options.range = Some(range);
        self
    }

    /// Set the If-Modified-Since header
    pub fn if_modified_since(mut self, time: impl Into<String>) -> Self {
        self.options.if_modified_since = Some(time.into());
        self
    }

    /// Set the If-Unmodified-Since header
    pub fn if_unmodified_since(mut self, time: impl Into<String>) -> Self {
        self.options.if_unmodified_since = Some(time.into());
        self
    }

    /// Set the If-Match header
    pub fn if_match(mut self, etag: impl Into<String>) -> Self {
        self.options.if_match = Some(etag.into());
        self
    }

    /// Set the If-None-Match header
    pub fn if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.options.if_none_match = Some(etag.into());
        self
    }

    /// Set the response Cache-Control header
    pub fn response_cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.params.response_cache_control = Some(cache_control.into());
        self
    }

    /// Set the response Content-Disposition header
    pub fn response_content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.params.response_content_disposition = Some(content_disposition.into());
        self
    }

    /// Set the response Content-Type header
    pub fn response_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.params.response_content_type = Some(content_type.into());
        self
    }

    /// Build parameters and options
    pub fn build(self) -> (GetObjectParams, Option<GetObjectOptions>) {
        let options = if self.options.range.is_some()
            || self.options.if_modified_since.is_some()
            || self.options.if_unmodified_since.is_some()
            || self.options.if_match.is_some()
            || self.options.if_none_match.is_some()
            || self.options.accept_encoding.is_some()
        {
            Some(self.options)
        } else {
            None
        };

        (self.params, options)
    }
}
