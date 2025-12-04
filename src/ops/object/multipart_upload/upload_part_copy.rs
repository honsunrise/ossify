use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use crate::body::ZeroBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Prepared, Request, escape_path};

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
    /// Specify the range of the copy source object
    pub copy_source_range: Option<(u64, u64)>,
    /// Copy condition for source object: execute copy operation if source object's ETag equals the user-provided ETag
    pub copy_source_if_match: Option<String>,
    /// Copy condition for source object: execute copy operation if source object's ETag does not equal the user-provided ETag
    pub copy_source_if_none_match: Option<String>,
    /// Copy condition for source object: transfer file normally if the time in the parameter is equal to or later than the actual file modification time
    pub copy_source_if_unmodified_since: Option<String>,
    /// Copy condition for source object: execute copy operation if source object was modified after the user-specified time
    pub copy_source_if_modified_since: Option<String>,
}

impl UploadPartCopyOptions {
    fn into_headers(
        self,
        source_bucket: String,
        source_key: String,
        source_version_id: Option<String>,
    ) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        let source_key = escape_path(&source_key);
        let mut source_str = format!("/{source_bucket}/{source_key}");
        if let Some(source_version_id) = source_version_id {
            source_str.push_str(&format!("?versionId={source_version_id}"));
        }
        headers.insert(HeaderName::from_static("x-oss-copy-source"), source_str.parse()?);

        if let Some(ref copy_source_range) = self.copy_source_range {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-range"),
                format!("bytes={}-{}", copy_source_range.0, copy_source_range.1).parse()?,
            );
        }

        if let Some(ref copy_source_if_match) = self.copy_source_if_match {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-match"),
                copy_source_if_match.to_string().parse()?,
            );
        }

        if let Some(ref copy_source_if_none_match) = self.copy_source_if_none_match {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-none-match"),
                copy_source_if_none_match.to_string().parse()?,
            );
        }

        if let Some(ref copy_source_if_modified_since) = self.copy_source_if_modified_since {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-modified-since"),
                copy_source_if_modified_since.to_string().parse()?,
            );
        }

        if let Some(ref copy_source_if_unmodified_since) = self.copy_source_if_unmodified_since {
            headers.insert(
                HeaderName::from_static("x-oss-copy-source-if-unmodified-since"),
                copy_source_if_unmodified_since.to_string().parse()?,
            );
        }

        Ok(headers)
    }
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
    pub source_version_id: Option<String>,
    pub params: UploadPartCopyParams,
    pub options: UploadPartCopyOptions,
}

impl Ops for UploadPartCopy {
    type Response = BodyResponseProcessor<UploadPartCopyResult>;
    type Body = ZeroBody;
    type Query = UploadPartCopyParams;

    fn prepare(self) -> Result<Prepared<UploadPartCopyParams>> {
        Ok(Prepared {
            method: Method::PUT,
            key: Some(self.object_key),
            query: Some(self.params),
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

/// Trait for UploadPartCopy operations
pub trait UploadPartCopyOperations {
    /// Upload part copy
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/uploadpartcopy>
    #[allow(clippy::too_many_arguments)]
    fn upload_part_copy(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        part_number: u32,
        source_bucket: impl Into<String>,
        source_key: impl Into<String>,
        options: Option<UploadPartCopyOptions>,
    ) -> impl Future<Output = Result<UploadPartCopyResult>>;

    /// Upload part copy
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/uploadpartcopy>
    #[allow(clippy::too_many_arguments)]
    fn upload_part_copy_with_version_id(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        part_number: u32,
        source_bucket: impl Into<String>,
        source_key: impl Into<String>,
        source_version_id: impl Into<String>,
        options: Option<UploadPartCopyOptions>,
    ) -> impl Future<Output = Result<UploadPartCopyResult>>;
}

impl UploadPartCopyOperations for Client {
    async fn upload_part_copy(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        part_number: u32,
        source_bucket: impl Into<String>,
        source_key: impl Into<String>,
        options: Option<UploadPartCopyOptions>,
    ) -> Result<UploadPartCopyResult> {
        let ops = UploadPartCopy {
            object_key: object_key.into(),
            source_bucket: source_bucket.into(),
            source_key: source_key.into(),
            source_version_id: None,
            params: UploadPartCopyParams::new(part_number, upload_id),
            options: options.unwrap_or_default(),
        };
        self.request(ops).await
    }

    async fn upload_part_copy_with_version_id(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        part_number: u32,
        source_bucket: impl Into<String>,
        source_key: impl Into<String>,
        source_version_id: impl Into<String>,
        options: Option<UploadPartCopyOptions>,
    ) -> Result<UploadPartCopyResult> {
        let ops = UploadPartCopy {
            object_key: object_key.into(),
            source_bucket: source_bucket.into(),
            source_key: source_key.into(),
            source_version_id: Some(source_version_id.into()),
            params: UploadPartCopyParams::new(part_number, upload_id),
            options: options.unwrap_or_default(),
        };
        self.request(ops).await
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
