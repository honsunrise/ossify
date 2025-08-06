use std::borrow::Cow;
use std::future::Future;

use http::header::HeaderName;
use http::{HeaderMap, Method};
use serde::{Deserialize, Serialize};

use crate::body::EmptyBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::utils::escape_path;
use crate::{Client, Ops, Request};

/// CopyObject request parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct CopyObjectParams {
    /// Version ID of the source object
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl CopyObjectParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

/// CopyObject options for headers
#[derive(Debug, Clone)]
pub struct CopyObjectOptions {
    /// Whether the CopyObject operation overwrites objects with the same name
    pub forbid_overwrite: Option<bool>,
    /// The object copy condition. If the ETag value of the source object is the same as the ETag value specified in the request, OSS copies the object
    pub copy_source_if_match: Option<String>,
    /// The object copy condition. If the ETag value of the source object is different from the ETag value specified in the request, OSS copies the object
    pub copy_source_if_none_match: Option<String>,
    /// The object copy condition. If the time specified in the request is the same as or later than the time when the object is modified, OSS copies the object
    pub copy_source_if_modified_since: Option<String>,
    /// The object copy condition. If the time specified in the request is earlier than the time when the object is modified, OSS copies the object
    pub copy_source_if_unmodified_since: Option<String>,
    /// The method that is used to set the metadata of the destination object
    pub metadata_directive: Option<String>,
    /// The cache control of the destination object
    pub cache_control: Option<String>,
    /// The content disposition of the destination object
    pub content_disposition: Option<String>,
    /// The content encoding of the destination object
    pub content_encoding: Option<String>,
    /// The content language of the destination object
    pub content_language: Option<String>,
    /// The content type of the destination object
    pub content_type: Option<String>,
    /// The expiration time of the destination object
    pub expires: Option<String>,
    /// The encryption method on the server side
    pub server_side_encryption: Option<String>,
    /// The CMK ID that is used for encryption on the server side
    pub server_side_encryption_key_id: Option<String>,
    /// The storage class of the destination object
    pub storage_class: Option<String>,
    /// The tag of the destination object
    pub tagging: Option<String>,
    /// The method that is used to configure tags for the destination object
    pub tagging_directive: Option<String>,
}

impl Default for CopyObjectOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl CopyObjectOptions {
    pub fn new() -> Self {
        Self {
            forbid_overwrite: None,
            copy_source_if_match: None,
            copy_source_if_none_match: None,
            copy_source_if_modified_since: None,
            copy_source_if_unmodified_since: None,
            metadata_directive: None,
            cache_control: None,
            content_disposition: None,
            content_encoding: None,
            content_language: None,
            content_type: None,
            expires: None,
            server_side_encryption: None,
            server_side_encryption_key_id: None,
            storage_class: None,
            tagging: None,
            tagging_directive: None,
        }
    }

    /// Set whether to forbid overwriting files with the same name
    pub fn forbid_overwrite(mut self, forbid_overwrite: bool) -> Self {
        self.forbid_overwrite = Some(forbid_overwrite);
        self
    }

    /// Set copy source if match condition
    pub fn copy_source_if_match(mut self, etag: impl Into<String>) -> Self {
        self.copy_source_if_match = Some(etag.into());
        self
    }

    /// Set copy source if none match condition
    pub fn copy_source_if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.copy_source_if_none_match = Some(etag.into());
        self
    }

    /// Set copy source if modified since condition
    pub fn copy_source_if_modified_since(mut self, time: impl Into<String>) -> Self {
        self.copy_source_if_modified_since = Some(time.into());
        self
    }

    /// Set copy source if unmodified since condition
    pub fn copy_source_if_unmodified_since(mut self, time: impl Into<String>) -> Self {
        self.copy_source_if_unmodified_since = Some(time.into());
        self
    }

    /// Set metadata directive
    pub fn metadata_directive(mut self, directive: impl Into<String>) -> Self {
        self.metadata_directive = Some(directive.into());
        self
    }

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

    /// Set content language
    pub fn content_language(mut self, content_language: impl Into<String>) -> Self {
        self.content_language = Some(content_language.into());
        self
    }

    /// Set content type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Set expires
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }

    /// Set server side encryption
    pub fn server_side_encryption(mut self, encryption: impl Into<String>) -> Self {
        self.server_side_encryption = Some(encryption.into());
        self
    }

    /// Set server side encryption key ID
    pub fn server_side_encryption_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.server_side_encryption_key_id = Some(key_id.into());
        self
    }

    /// Set storage class
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.storage_class = Some(storage_class.into());
        self
    }

    /// Set tagging
    pub fn tagging(mut self, tagging: impl Into<String>) -> Self {
        self.tagging = Some(tagging.into());
        self
    }

    /// Set tagging directive
    pub fn tagging_directive(mut self, directive: impl Into<String>) -> Self {
        self.tagging_directive = Some(directive.into());
        self
    }
}

/// CopyObject response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CopyObjectResult {
    /// The ETag value of the destination object
    #[serde(rename = "ETag")]
    pub etag: String,
    /// The time when the destination object was last modified
    pub last_modified: String,
}

/// CopyObject operation
pub struct CopyObject {
    pub source_key_with_bucket: String,
    pub target_key: String,
    pub params: Option<CopyObjectParams>,
    pub options: Option<CopyObjectOptions>,
}

impl Ops for CopyObject {
    type Response = BodyResponseProcessor<CopyObjectResult>;
    type Body = EmptyBody;
    type Query = CopyObjectParams;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn key<'a>(&'a self) -> Option<Cow<'a, str>> {
        Some(Cow::Borrowed(&self.target_key))
    }

    fn query(&self) -> Option<&Self::Query> {
        self.params.as_ref()
    }

    fn headers(&self) -> Result<Option<HeaderMap>> {
        let mut headers = HeaderMap::new();

        // Set copy source (required)
        let copy_source = escape_path(&self.source_key_with_bucket);
        headers.insert(HeaderName::from_static("x-oss-copy-source"), copy_source.parse()?);

        if let Some(options) = &self.options {
            // Set forbid overwrite
            if let Some(forbid_overwrite) = &options.forbid_overwrite {
                headers.insert(
                    HeaderName::from_static("x-oss-forbid-overwrite"),
                    forbid_overwrite.to_string().parse()?,
                );
            }

            // Set copy source conditions
            if let Some(copy_source_if_match) = &options.copy_source_if_match {
                headers.insert(
                    HeaderName::from_static("x-oss-copy-source-if-match"),
                    copy_source_if_match.parse()?,
                );
            }

            if let Some(copy_source_if_none_match) = &options.copy_source_if_none_match {
                headers.insert(
                    HeaderName::from_static("x-oss-copy-source-if-none-match"),
                    copy_source_if_none_match.parse()?,
                );
            }

            if let Some(copy_source_if_modified_since) = &options.copy_source_if_modified_since {
                headers.insert(
                    HeaderName::from_static("x-oss-copy-source-if-modified-since"),
                    copy_source_if_modified_since.parse()?,
                );
            }

            if let Some(copy_source_if_unmodified_since) = &options.copy_source_if_unmodified_since {
                headers.insert(
                    HeaderName::from_static("x-oss-copy-source-if-unmodified-since"),
                    copy_source_if_unmodified_since.parse()?,
                );
            }

            // Set metadata directive
            if let Some(metadata_directive) = &options.metadata_directive {
                headers
                    .insert(HeaderName::from_static("x-oss-metadata-directive"), metadata_directive.parse()?);
            }

            // Set content headers
            if let Some(cache_control) = &options.cache_control {
                headers.insert(HeaderName::from_static("cache-control"), cache_control.parse()?);
            }

            if let Some(content_disposition) = &options.content_disposition {
                headers.insert(HeaderName::from_static("content-disposition"), content_disposition.parse()?);
            }

            if let Some(content_encoding) = &options.content_encoding {
                headers.insert(HeaderName::from_static("content-encoding"), content_encoding.parse()?);
            }

            if let Some(content_language) = &options.content_language {
                headers.insert(HeaderName::from_static("content-language"), content_language.parse()?);
            }

            if let Some(content_type) = &options.content_type {
                headers.insert(HeaderName::from_static("content-type"), content_type.parse()?);
            }

            if let Some(expires) = &options.expires {
                headers.insert(HeaderName::from_static("expires"), expires.parse()?);
            }

            // Set server side encryption
            if let Some(server_side_encryption) = &options.server_side_encryption {
                headers.insert(
                    HeaderName::from_static("x-oss-server-side-encryption"),
                    server_side_encryption.parse()?,
                );
            }

            if let Some(server_side_encryption_key_id) = &options.server_side_encryption_key_id {
                headers.insert(
                    HeaderName::from_static("x-oss-server-side-encryption-key-id"),
                    server_side_encryption_key_id.parse()?,
                );
            }

            // Set storage class
            if let Some(storage_class) = &options.storage_class {
                headers.insert(HeaderName::from_static("x-oss-storage-class"), storage_class.parse()?);
            }

            // Set tagging
            if let Some(tagging) = &options.tagging {
                headers.insert(HeaderName::from_static("x-oss-tagging"), tagging.parse()?);
            }

            if let Some(tagging_directive) = &options.tagging_directive {
                headers
                    .insert(HeaderName::from_static("x-oss-tagging-directive"), tagging_directive.parse()?);
            }
        }
        Ok(Some(headers))
    }
}

/// Trait for CopyObject operations
pub trait CopyObjectOperations {
    /// Copy an object within a bucket or between buckets in the same region
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/copyobject>
    fn copy_object(
        &self,
        source_key_with_bucket: impl Into<String>,
        target_key: impl Into<String>,
        params: Option<CopyObjectParams>,
        options: Option<CopyObjectOptions>,
    ) -> impl Future<Output = Result<CopyObjectResult>>;
}

impl CopyObjectOperations for Client {
    async fn copy_object(
        &self,
        source_key_with_bucket: impl Into<String>,
        target_key: impl Into<String>,
        params: Option<CopyObjectParams>,
        options: Option<CopyObjectOptions>,
    ) -> Result<CopyObjectResult> {
        let source_key_with_bucket = source_key_with_bucket.into();
        let target_key = target_key.into();

        let ops = CopyObject {
            source_key_with_bucket,
            target_key,
            params,
            options,
        };

        self.request(ops).await
    }
}
