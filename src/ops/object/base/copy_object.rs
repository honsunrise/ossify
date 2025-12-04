use std::future::Future;

use http::header::HeaderName;
use http::{HeaderMap, Method};
use serde::Deserialize;

use crate::body::ZeroBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::utils::escape_path;
use crate::{Client, Ops, Prepared, Request};

/// CopyObject options for headers
#[derive(Debug, Clone, Default)]
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

impl CopyObjectOptions {
    pub fn new() -> Self {
        Default::default()
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

impl CopyObjectOptions {
    fn into_headers(
        self,
        source_bucket: String,
        source_key: String,
        source_version_id: Option<String>,
    ) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        // Set copy source (required)
        let source_key = escape_path(&source_key);
        let mut copy_source = format!("/{source_bucket}/{source_key}");
        if let Some(source_version_id) = source_version_id {
            copy_source.push_str(&format!("?versionId={source_version_id}"));
        }
        headers.insert(HeaderName::from_static("x-oss-copy-source"), copy_source.parse()?);

        // Set forbid overwrite
        if let Some(forbid_overwrite) = &self.forbid_overwrite {
            headers.insert(
                HeaderName::from_static("x-oss-forbid-overwrite"),
                forbid_overwrite.to_string().parse()?,
            );
        }

        // Set copy source conditions
        if let Some(copy_source_if_match) = &self.copy_source_if_match {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-match"),
                copy_source_if_match.parse()?,
            );
        }

        if let Some(copy_source_if_none_match) = &self.copy_source_if_none_match {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-none-match"),
                copy_source_if_none_match.parse()?,
            );
        }

        if let Some(copy_source_if_modified_since) = &self.copy_source_if_modified_since {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-modified-since"),
                copy_source_if_modified_since.parse()?,
            );
        }

        if let Some(copy_source_if_unmodified_since) = &self.copy_source_if_unmodified_since {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-unmodified-since"),
                copy_source_if_unmodified_since.parse()?,
            );
        }

        // Set metadata directive
        if let Some(metadata_directive) = &self.metadata_directive {
            headers.insert(HeaderName::from_static("x-oss-metadata-directive"), metadata_directive.parse()?);
        }

        // Set content headers
        if let Some(cache_control) = &self.cache_control {
            headers.insert(HeaderName::from_static("cache-control"), cache_control.parse()?);
        }

        if let Some(content_disposition) = &self.content_disposition {
            headers.insert(HeaderName::from_static("content-disposition"), content_disposition.parse()?);
        }

        if let Some(content_encoding) = &self.content_encoding {
            headers.insert(HeaderName::from_static("content-encoding"), content_encoding.parse()?);
        }

        if let Some(content_language) = &self.content_language {
            headers.insert(HeaderName::from_static("content-language"), content_language.parse()?);
        }

        if let Some(content_type) = &self.content_type {
            headers.insert(HeaderName::from_static("content-type"), content_type.parse()?);
        }

        if let Some(expires) = &self.expires {
            headers.insert(HeaderName::from_static("expires"), expires.parse()?);
        }

        // Set server side encryption
        if let Some(server_side_encryption) = &self.server_side_encryption {
            headers.insert(
                HeaderName::from_static("x-oss-server-side-encryption"),
                server_side_encryption.parse()?,
            );
        }

        if let Some(server_side_encryption_key_id) = &self.server_side_encryption_key_id {
            headers.insert(
                HeaderName::from_static("x-oss-server-side-encryption-key-id"),
                server_side_encryption_key_id.parse()?,
            );
        }

        // Set storage class
        if let Some(storage_class) = &self.storage_class {
            headers.insert(HeaderName::from_static("x-oss-storage-class"), storage_class.parse()?);
        }

        // Set tagging
        if let Some(tagging) = &self.tagging {
            headers.insert(HeaderName::from_static("x-oss-tagging"), tagging.parse()?);
        }

        if let Some(tagging_directive) = &self.tagging_directive {
            headers.insert(HeaderName::from_static("x-oss-tagging-directive"), tagging_directive.parse()?);
        }
        Ok(headers)
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
    pub source_bucket: String,
    pub source_key: String,
    pub source_version_id: Option<String>,
    pub target_key: String,
    pub options: CopyObjectOptions,
}

impl Ops for CopyObject {
    type Response = BodyResponseProcessor<CopyObjectResult>;
    type Body = ZeroBody;
    type Query = ();

    fn prepare(self) -> Result<Prepared> {
        Ok(Prepared {
            method: Method::PUT,
            key: Some(self.target_key),
            headers: Some(self.options.into_headers(
                self.source_bucket,
                self.source_key,
                self.source_version_id,
            )?),
            body: Some(()),
            ..Default::default()
        })
    }
}

/// Trait for CopyObject operations
pub trait CopyObjectOperations {
    /// Copy an object within a bucket or between buckets in the same region
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/copyobject>
    fn copy_object(
        &self,
        source_bucket: impl Into<String>,
        source_key: impl Into<String>,
        target_key: impl Into<String>,
        options: Option<CopyObjectOptions>,
    ) -> impl Future<Output = Result<CopyObjectResult>>;

    /// Copy an object within a bucket or between buckets in the same region
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/copyobject>
    fn copy_object_with_version_id(
        &self,
        source_bucket: impl Into<String>,
        source_key: impl Into<String>,
        source_version_id: impl Into<String>,
        target_key: impl Into<String>,
        options: Option<CopyObjectOptions>,
    ) -> impl Future<Output = Result<CopyObjectResult>>;
}

impl CopyObjectOperations for Client {
    async fn copy_object(
        &self,
        source_bucket: impl Into<String>,
        source_key: impl Into<String>,
        target_key: impl Into<String>,
        options: Option<CopyObjectOptions>,
    ) -> Result<CopyObjectResult> {
        let source_bucket = source_bucket.into();
        let source_key = source_key.into();
        let target_key = target_key.into();

        let ops = CopyObject {
            source_bucket,
            source_key,
            source_version_id: None,
            target_key,
            options: options.unwrap_or_default(),
        };

        self.request(ops).await
    }

    async fn copy_object_with_version_id(
        &self,
        source_bucket: impl Into<String>,
        source_key: impl Into<String>,
        source_version_id: impl Into<String>,
        target_key: impl Into<String>,
        options: Option<CopyObjectOptions>,
    ) -> Result<CopyObjectResult> {
        let source_bucket = source_bucket.into();
        let source_key = source_key.into();
        let source_version_id = source_version_id.into();
        let target_key = target_key.into();
        let options = options.unwrap_or_default();

        let ops = CopyObject {
            source_bucket,
            source_key,
            source_version_id: Some(source_version_id),
            target_key,
            options,
        };

        self.request(ops).await
    }
}
