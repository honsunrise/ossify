use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;

use bytes::Bytes;
use heck::ToKebabCase;
use http::{HeaderMap, HeaderName, Method, header};
use serde::{Deserialize, Serialize};

use super::{ServerSideEncryption, StorageClass};
use crate::body::BinaryBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::{Client, Ops, Request};

/// PutObject request parameters (query parameters)
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutObjectParams {
    /// Version ID (used to overwrite a specific version, rarely used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl PutObjectParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

/// PutObject request options (mainly set via HTTP headers)
#[derive(Debug, Clone, Default)]
pub struct PutObjectOptions {
    /// Cache control
    pub cache_control: Option<String>,
    /// Content disposition (filename when downloading, etc.)
    pub content_disposition: Option<String>,
    /// Content encoding
    pub content_encoding: Option<String>,
    /// Content type
    pub content_type: Option<String>,
    /// Expiration time
    pub expires: Option<String>,
    /// Whether to forbid overwriting files with the same name
    pub forbid_overwrite: Option<bool>,
    /// Storage class
    pub storage_class: Option<StorageClass>,
    /// Server-side encryption method
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// Server-side encryption key ID (used by KMS)
    pub server_side_encryption_key_id: Option<String>,
    /// Object access control list
    pub object_acl: Option<String>,
    /// User-defined metadata
    pub user_meta: HashMap<String, String>,
    /// Object tags
    pub tagging: HashMap<String, String>,
    /// Content MD5 value
    pub content_md5: Option<String>,
}

impl PutObjectOptions {
    /// Set cache control
    pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.cache_control = Some(cache_control.into());
        self
    }

    /// Set content disposition
    pub fn content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.content_disposition = Some(content_disposition.into());
        self
    }

    /// Set content encoding
    pub fn content_encoding(mut self, content_encoding: impl Into<String>) -> Self {
        self.content_encoding = Some(content_encoding.into());
        self
    }

    /// Set content type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Set expiration time
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }

    /// Set whether to forbid overwriting
    pub fn forbid_overwrite(mut self, forbid: bool) -> Self {
        self.forbid_overwrite = Some(forbid);
        self
    }

    /// Set storage class
    pub fn storage_class(mut self, storage_class: StorageClass) -> Self {
        self.storage_class = Some(storage_class);
        self
    }

    /// Set server-side encryption
    pub fn server_side_encryption(mut self, encryption: ServerSideEncryption) -> Self {
        self.server_side_encryption = Some(encryption);
        self
    }

    /// Set KMS key ID
    pub fn server_side_encryption_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.server_side_encryption_key_id = Some(key_id.into());
        self
    }

    /// Set object ACL
    pub fn object_acl(mut self, acl: impl Into<String>) -> Self {
        self.object_acl = Some(acl.into());
        self
    }

    /// Add user metadata
    pub fn user_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.user_meta.insert(key.into(), value.into());
        self
    }

    /// Set user metadata in batch
    pub fn user_meta_map(mut self, meta: HashMap<String, String>) -> Self {
        self.user_meta.extend(meta);
        self
    }

    /// Add object tag
    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tagging.insert(key.into(), value.into());
        self
    }

    /// Set object tags in batch
    pub fn tagging_map(mut self, tags: HashMap<String, String>) -> Self {
        self.tagging.extend(tags);
        self
    }

    /// Set Content-MD5
    pub fn content_md5(mut self, md5: impl Into<String>) -> Self {
        self.content_md5 = Some(md5.into());
        self
    }
}

/// PutObject response (mainly obtained from response headers)
#[derive(Debug, Clone, Deserialize)]
pub struct PutObjectResponse {
    /// ETag value
    #[serde(rename = "etag")]
    pub etag: String,
    /// Version ID (if versioning is enabled)
    #[serde(rename = "x-oss-version-id")]
    pub version_id: Option<String>,
    /// CRC64 value
    #[serde(rename = "x-oss-hash-crc64ecma")]
    pub hash_crc64ecma: Option<String>,
    /// Server-side encryption method
    #[serde(rename = "x-oss-server-side-encryption")]
    pub server_side_encryption: Option<String>,
    /// Server-side encryption key ID
    #[serde(rename = "x-oss-server-side-encryption-key-id")]
    pub server_side_encryption_key_id: Option<String>,
}

/// PutObject operation
pub struct PutObject {
    pub object_key: String,
    pub params: PutObjectParams,
    pub options: Option<PutObjectOptions>,
    pub body: Bytes,
}

impl Ops for PutObject {
    type Response = HeaderResponseProcessor<PutObjectResponse>;
    type Body = BinaryBody;
    type Query = PutObjectParams;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn key<'a>(&'a self) -> Option<Cow<'a, str>> {
        Some(Cow::Borrowed(&self.object_key))
    }

    fn query(&self) -> Option<&Self::Query> {
        if self.params.version_id.is_some() {
            Some(&self.params)
        } else {
            None
        }
    }

    fn body(&self) -> Option<&Bytes> {
        Some(&self.body)
    }

    fn headers(&self) -> Result<Option<HeaderMap>> {
        let mut headers = HeaderMap::new();
        let Some(options) = &self.options else {
            return Ok(None);
        };

        // Set cache control
        if let Some(cache_control) = &options.cache_control {
            headers.insert(header::CACHE_CONTROL, cache_control.parse()?);
        }

        // Set content disposition
        if let Some(content_disposition) = &options.content_disposition {
            headers.insert(header::CONTENT_DISPOSITION, content_disposition.parse()?);
        }

        // Set content encoding
        if let Some(content_encoding) = &options.content_encoding {
            headers.insert(header::CONTENT_ENCODING, content_encoding.parse()?);
        }

        // Set content type (overrides the default value set by BinaryBody)
        if let Some(content_type) = &options.content_type {
            headers.insert(header::CONTENT_TYPE, content_type.parse()?);
        }

        // Set expiration time
        if let Some(expires) = &options.expires {
            headers.insert(header::EXPIRES, expires.parse()?);
        }

        // Set Content-MD5
        if let Some(content_md5) = &options.content_md5 {
            headers.insert(HeaderName::from_static("content-md5"), content_md5.parse()?);
        }

        // Set whether to forbid overwriting files with the same name
        if let Some(forbid_overwrite) = &options.forbid_overwrite {
            headers.insert(
                HeaderName::from_static("x-oss-forbid-overwrite"),
                forbid_overwrite.to_string().parse()?,
            );
        }

        // Set storage class
        if let Some(storage_class) = &options.storage_class {
            headers.insert(HeaderName::from_static("x-oss-storage-class"), storage_class.as_ref().parse()?);
        }

        // Set server-side encryption method
        if let Some(encryption) = &options.server_side_encryption {
            headers.insert(
                HeaderName::from_static("x-oss-server-side-encryption"),
                encryption.as_ref().parse()?,
            );
        }

        // Set KMS key ID
        if let Some(key_id) = &options.server_side_encryption_key_id {
            headers.insert(HeaderName::from_static("x-oss-server-side-encryption-key-id"), key_id.parse()?);
        }

        // Set object ACL
        if let Some(acl) = &options.object_acl {
            headers.insert(HeaderName::from_static("x-oss-object-acl"), acl.parse()?);
        }

        // Set user-defined metadata
        for (key, value) in &options.user_meta {
            let key = key.to_kebab_case().to_lowercase();
            let header_name = format!("x-oss-meta-{key}");
            headers.insert(HeaderName::from_bytes(header_name.as_bytes())?, value.parse()?);
        }

        // Set object tags
        if !options.tagging.is_empty() {
            let tagging_str = options
                .tagging
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            headers.insert(HeaderName::from_static("x-oss-tagging"), tagging_str.parse()?);
        }

        if headers.is_empty() {
            Ok(None)
        } else {
            Ok(Some(headers))
        }
    }
}

/// Trait for PutObject operations
pub trait PutObjectOperations {
    /// Upload an object to OSS
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/putobject>
    fn put_object(
        &self,
        object_key: impl AsRef<str>,
        data: &[u8],
        options: Option<PutObjectOptions>,
    ) -> impl Future<Output = Result<PutObjectResponse>>;
}

impl PutObjectOperations for Client {
    async fn put_object(
        &self,
        object_key: impl AsRef<str>,
        data: &[u8],
        options: Option<PutObjectOptions>,
    ) -> Result<PutObjectResponse> {
        let ops = PutObject {
            object_key: object_key.as_ref().to_string(),
            params: PutObjectParams::new(),
            options,
            body: Bytes::copy_from_slice(data),
        };

        self.request(ops).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// PutObject request builder
#[derive(Debug, Clone, Default)]
pub struct PutObjectRequestBuilder {
    options: PutObjectOptions,
}

impl PutObjectRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set cache control
    pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.options.cache_control = Some(cache_control.into());
        self
    }

    /// Set content disposition
    pub fn content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.options.content_disposition = Some(content_disposition.into());
        self
    }

    /// Set content encoding
    pub fn content_encoding(mut self, content_encoding: impl Into<String>) -> Self {
        self.options.content_encoding = Some(content_encoding.into());
        self
    }

    /// Set content type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.options.content_type = Some(content_type.into());
        self
    }

    /// Set expiration time
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.options.expires = Some(expires.into());
        self
    }

    /// Set whether to forbid overwriting
    pub fn forbid_overwrite(mut self, forbid: bool) -> Self {
        self.options.forbid_overwrite = Some(forbid);
        self
    }

    /// Set storage class
    pub fn storage_class(mut self, storage_class: StorageClass) -> Self {
        self.options.storage_class = Some(storage_class);
        self
    }

    /// Set server-side encryption
    pub fn server_side_encryption(mut self, encryption: ServerSideEncryption) -> Self {
        self.options.server_side_encryption = Some(encryption);
        self
    }

    /// Set KMS key ID
    pub fn server_side_encryption_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.options.server_side_encryption_key_id = Some(key_id.into());
        self
    }

    /// Set object ACL
    pub fn object_acl(mut self, acl: impl Into<String>) -> Self {
        self.options.object_acl = Some(acl.into());
        self
    }

    /// Add user metadata
    pub fn user_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.user_meta.insert(key.into(), value.into());
        self
    }

    /// Add object tag
    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.tagging.insert(key.into(), value.into());
        self
    }

    /// Set Content-MD5
    pub fn content_md5(mut self, md5: impl Into<String>) -> Self {
        self.options.content_md5 = Some(md5.into());
        self
    }

    /// Build options
    pub fn build(self) -> PutObjectOptions {
        self.options
    }
}
