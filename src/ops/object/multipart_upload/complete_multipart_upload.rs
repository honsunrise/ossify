use std::borrow::Cow;
use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

/// CompleteMultipartUpload request parameters
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteMultipartUploadParams {
    pub upload_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

impl CompleteMultipartUploadParams {
    pub fn new(upload_id: impl Into<String>) -> Self {
        Self {
            upload_id: upload_id.into(),
            encoding_type: None,
        }
    }

    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.encoding_type = Some(encoding_type.into());
        self
    }
}

/// CompleteMultipartUpload request options
#[derive(Debug, Clone, Default)]
pub struct CompleteMultipartUploadOptions {
    /// Whether to forbid overwriting objects with the same name
    pub forbid_overwrite: Option<bool>,
    /// Whether to automatically complete all parts (server lists and sorts)
    pub complete_all: Option<bool>,
    /// Object access permissions
    pub object_acl: Option<String>,
}

/// Part information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Part {
    /// Part number
    pub part_number: u32,
    /// ETag value of the part
    #[serde(rename = "ETag")]
    pub etag: String,
}

impl Part {
    pub fn new(part_number: u32, etag: impl Into<String>) -> Self {
        Self {
            part_number,
            etag: etag.into(),
        }
    }
}

/// CompleteMultipartUpload request body
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename = "CompleteMultipartUpload")]
pub struct CompleteMultipartUploadBody {
    #[serde(rename = "Part", default)]
    pub parts: Vec<Part>,
}

impl CompleteMultipartUploadBody {
    pub fn new(parts: Vec<Part>) -> Self {
        Self { parts }
    }

    pub fn add_part(&mut self, part: Part) {
        self.parts.push(part);
        // Ensure sorting by part number
        self.parts.sort_by_key(|p| p.part_number);
    }

    pub fn add_parts(&mut self, mut parts: Vec<Part>) {
        self.parts.append(&mut parts);
        // Ensure sorting by part number
        self.parts.sort_by_key(|p| p.part_number);
    }
}

/// CompleteMultipartUpload response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompleteMultipartUploadResult {
    /// Bucket name
    pub bucket: String,
    /// Object name
    pub key: String,
    /// ETag value of the object
    #[serde(rename = "ETag")]
    pub etag: String,
    /// URL of the object
    pub location: String,
    /// Encoding type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

/// CompleteMultipartUpload operation
pub struct CompleteMultipartUpload {
    pub object_key: String,
    pub params: CompleteMultipartUploadParams,
    pub body: Option<CompleteMultipartUploadBody>,
    pub options: Option<CompleteMultipartUploadOptions>,
}

impl Ops for CompleteMultipartUpload {
    type Response = BodyResponseProcessor<CompleteMultipartUploadResult>;
    type Body = XMLBody<CompleteMultipartUploadBody>;
    type Query = CompleteMultipartUploadParams;

    const PRODUCT: &'static str = "oss";

    fn method(&self) -> Method {
        Method::POST
    }

    fn key<'a>(&'a self) -> Option<Cow<'a, str>> {
        Some(Cow::Borrowed(&self.object_key))
    }

    fn headers(&self) -> Result<Option<HeaderMap>> {
        let mut headers = HeaderMap::new();
        let Some(options) = &self.options else {
            return Ok(None);
        };

        if let Some(forbid_overwrite) = &options.forbid_overwrite {
            headers.insert(
                HeaderName::from_static("x-oss-forbid-overwrite"),
                forbid_overwrite.to_string().parse()?,
            );
        }

        if let Some(object_acl) = &options.object_acl {
            headers.insert(HeaderName::from_static("x-oss-object-acl"), object_acl.parse()?);
        }

        if let Some(complete_all) = &options.complete_all {
            headers.insert(HeaderName::from_static("x-oss-complete-all"), complete_all.to_string().parse()?);
        }

        Ok(Some(headers))
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.params)
    }

    fn body(&self) -> Option<&CompleteMultipartUploadBody> {
        self.body.as_ref()
    }
}

/// Trait for CompleteMultipartUpload operations
pub trait CompleteMultipartUploadOperations {
    /// Complete multipart upload
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/completemultipartupload>
    fn complete_multipart_upload(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        parts: Vec<Part>,
        options: Option<CompleteMultipartUploadOptions>,
    ) -> impl Future<Output = Result<CompleteMultipartUploadResult>>;

    /// Automatically complete multipart upload (server lists and sorts all parts)
    fn complete_multipart_upload_auto(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        options: Option<CompleteMultipartUploadOptions>,
    ) -> impl Future<Output = Result<CompleteMultipartUploadResult>>;
}

impl CompleteMultipartUploadOperations for Client {
    async fn complete_multipart_upload(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        parts: Vec<Part>,
        options: Option<CompleteMultipartUploadOptions>,
    ) -> Result<CompleteMultipartUploadResult> {
        let mut sorted_parts = parts;
        sorted_parts.sort_by_key(|p| p.part_number);

        let ops = CompleteMultipartUpload {
            object_key: object_key.as_ref().to_string(),
            params: CompleteMultipartUploadParams::new(upload_id.as_ref()),
            body: Some(CompleteMultipartUploadBody::new(sorted_parts)),
            options,
        };
        self.request(ops).await
    }

    async fn complete_multipart_upload_auto(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        options: Option<CompleteMultipartUploadOptions>,
    ) -> Result<CompleteMultipartUploadResult> {
        let mut auto_options = options.unwrap_or_default();
        auto_options.complete_all = Some(true);

        let ops = CompleteMultipartUpload {
            object_key: object_key.as_ref().to_string(),
            params: CompleteMultipartUploadParams::new(upload_id.as_ref()),
            body: None, // No need to provide body when auto-completing
            options: Some(auto_options),
        };
        self.request(ops).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// CompleteMultipartUpload request builder
#[derive(Debug, Clone, Default)]
pub struct CompleteMultipartUploadRequestBuilder {
    options: CompleteMultipartUploadOptions,
}

impl CompleteMultipartUploadRequestBuilder {
    /// Create a new request builder
    pub fn new() -> Self {
        Self {
            options: CompleteMultipartUploadOptions::default(),
        }
    }

    /// Set whether to forbid overwriting objects with the same name
    pub fn forbid_overwrite(mut self, forbid: bool) -> Self {
        self.options.forbid_overwrite = Some(forbid);
        self
    }

    /// Set whether to automatically complete all parts
    pub fn complete_all(mut self, complete_all: bool) -> Self {
        self.options.complete_all = Some(complete_all);
        self
    }

    /// Set object access permissions
    pub fn object_acl(mut self, acl: impl Into<String>) -> Self {
        self.options.object_acl = Some(acl.into());
        self
    }

    /// Build request options
    pub fn build(self) -> CompleteMultipartUploadOptions {
        self.options
    }
}
