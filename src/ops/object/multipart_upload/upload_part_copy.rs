use std::borrow::Cow;
use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use crate::body::EmptyBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

/// UploadPartCopy request parameters
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadPartCopyParams {
    pub part_number: u32,
    pub upload_id: String,
}

impl UploadPartCopyParams {
    pub fn new(part_number: u32, upload_id: impl Into<String>) -> Self {
        Self {
            part_number,
            upload_id: upload_id.into(),
        }
    }
}

/// UploadPartCopy request options
#[derive(Debug, Clone, Default)]
pub struct UploadPartCopyOptions {
    /// Copy condition for source object: execute copy operation if source object's ETag equals the user-provided ETag
    pub copy_source_if_match: Option<String>,
    /// Copy condition for source object: execute copy operation if source object's ETag does not equal the user-provided ETag
    pub copy_source_if_none_match: Option<String>,
    /// Copy condition for source object: transfer file normally if the time in the parameter is equal to or later than the actual file modification time
    pub copy_source_if_unmodified_since: Option<String>,
    /// Copy condition for source object: execute copy operation if source object was modified after the user-specified time
    pub copy_source_if_modified_since: Option<String>,
    /// Specify the copy source object
    pub copy_source: Option<String>,
    /// Specify the range of the copy source object
    pub copy_source_range: Option<(u64, u64)>,
}

/// Copy result in UploadPartCopy response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CopyPartResult {
    /// ETag value of the newly created object
    pub etag: String,
    /// Last modification time of the object
    pub last_modified: String,
}

/// UploadPartCopy response
#[derive(Debug, Clone, Deserialize)]
pub struct UploadPartCopyResult {
    /// Copy result
    pub copy_part_result: CopyPartResult,
    /// Part number
    pub part_number: u32,
}

/// UploadPartCopy operation
pub struct UploadPartCopy {
    pub object_key: String,
    pub source_bucket: String,
    pub source_key: String,
    pub params: UploadPartCopyParams,
    pub options: Option<UploadPartCopyOptions>,
}

impl Ops for UploadPartCopy {
    type Response = BodyResponseProcessor<UploadPartCopyResult>;
    type Body = EmptyBody;
    type Query = UploadPartCopyParams;

    const PRODUCT: &'static str = "oss";

    fn method(&self) -> Method {
        Method::PUT
    }

    fn headers(&self) -> Result<Option<HeaderMap>> {
        let mut headers = HeaderMap::new();
        let Some(ref options) = self.options else {
            return Ok(None);
        };

        if let Some(ref copy_source) = options.copy_source {
            headers.insert(HeaderName::from_static("x-oss-copy-source"), copy_source.parse()?);
        }

        if let Some(ref copy_source_range) = options.copy_source_range {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-range"),
                format!("bytes={}-{}", copy_source_range.0, copy_source_range.1).parse()?,
            );
        }

        if let Some(ref copy_source_if_match) = options.copy_source_if_match {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-match"),
                copy_source_if_match.to_string().parse()?,
            );
        }

        if let Some(ref copy_source_if_none_match) = options.copy_source_if_none_match {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-none-match"),
                copy_source_if_none_match.to_string().parse()?,
            );
        }

        if let Some(ref copy_source_if_modified_since) = options.copy_source_if_modified_since {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-modified-since"),
                copy_source_if_modified_since.to_string().parse()?,
            );
        }

        if let Some(ref copy_source_if_unmodified_since) = options.copy_source_if_unmodified_since {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-unmodified-since"),
                copy_source_if_unmodified_since.to_string().parse()?,
            );
        }

        Ok(Some(headers))
    }

    fn key<'a>(&'a self) -> Option<Cow<'a, str>> {
        Some(Cow::Borrowed(&self.object_key))
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.params)
    }
}

/// Trait for UploadPartCopy operations
pub trait UploadPartCopyOperations {
    /// Upload part copy
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/uploadpartcopy>
    #[allow(clippy::too_many_arguments)]
    fn upload_part_copy(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        part_number: u32,
        source_bucket: impl AsRef<str>,
        source_key: impl AsRef<str>,
        options: Option<UploadPartCopyOptions>,
    ) -> impl Future<Output = Result<UploadPartCopyResult>>;

    /// Upload part copy from a specific version of the source object
    #[allow(clippy::too_many_arguments)]
    fn upload_part_copy_with_version(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        part_number: u32,
        source_bucket: impl AsRef<str>,
        source_key: impl AsRef<str>,
        version_id: impl AsRef<str>,
        options: Option<UploadPartCopyOptions>,
    ) -> impl Future<Output = Result<UploadPartCopyResult>>;
}

impl UploadPartCopyOperations for Client {
    async fn upload_part_copy(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        part_number: u32,
        source_bucket: impl AsRef<str>,
        source_key: impl AsRef<str>,
        options: Option<UploadPartCopyOptions>,
    ) -> Result<UploadPartCopyResult> {
        let ops = UploadPartCopy {
            object_key: object_key.as_ref().to_string(),
            source_bucket: source_bucket.as_ref().to_string(),
            source_key: source_key.as_ref().to_string(),
            params: UploadPartCopyParams::new(part_number, upload_id.as_ref()),
            options,
        };
        self.request(ops).await
    }

    async fn upload_part_copy_with_version(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        part_number: u32,
        source_bucket: impl AsRef<str>,
        source_key: impl AsRef<str>,
        version_id: impl AsRef<str>,
        options: Option<UploadPartCopyOptions>,
    ) -> Result<UploadPartCopyResult> {
        // For versioned source objects, need to append ?versionId=xxx after source_key
        let versioned_source_key = format!("{}?versionId={}", source_key.as_ref(), version_id.as_ref());

        let ops = UploadPartCopy {
            object_key: object_key.as_ref().to_string(),
            source_bucket: source_bucket.as_ref().to_string(),
            source_key: versioned_source_key,
            params: UploadPartCopyParams::new(part_number, upload_id.as_ref()),
            options,
        };
        self.request(ops).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// UploadPartCopy request builder
#[derive(Debug, Clone, Default)]
pub struct UploadPartCopyRequestBuilder {
    options: UploadPartCopyOptions,
}

impl UploadPartCopyRequestBuilder {
    /// Create a new request builder
    pub fn new() -> Self {
        Self {
            options: UploadPartCopyOptions::default(),
        }
    }

    /// Set the range of the copy source object
    pub fn copy_source(mut self, source: impl Into<String>) -> Self {
        self.options.copy_source = Some(source.into());
        self
    }

    /// Convenience method: set byte range
    pub fn copy_source_byte_range(mut self, start: u64, end: u64) -> Self {
        self.options.copy_source_range = Some((start, end));
        self
    }

    /// Build request options
    pub fn build(self) -> UploadPartCopyOptions {
        self.options
    }
}

/// Part copy information, used for completing multipart upload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PartCopyInfo {
    pub part_number: u32,
    pub etag: String,
}

impl PartCopyInfo {
    pub fn new(part_number: u32, etag: impl Into<String>) -> Self {
        Self {
            part_number,
            etag: etag.into(),
        }
    }
}

impl From<UploadPartCopyResult> for PartCopyInfo {
    fn from(result: UploadPartCopyResult) -> Self {
        Self {
            part_number: result.part_number,
            etag: result.copy_part_result.etag,
        }
    }
}
