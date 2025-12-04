use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// ListParts request parameters
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListPartsParams {
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    pub max_parts: Option<u32>,
    pub part_number_marker: Option<u32>,
    pub encoding_type: Option<String>,
}

impl ListPartsParams {
    pub fn new(upload_id: impl Into<String>) -> Self {
        Self {
            upload_id: upload_id.into(),
            max_parts: None,
            part_number_marker: None,
            encoding_type: None,
        }
    }

    pub fn max_parts(mut self, max_parts: u32) -> Self {
        self.max_parts = Some(max_parts);
        self
    }

    pub fn part_number_marker(mut self, part_number_marker: u32) -> Self {
        self.part_number_marker = Some(part_number_marker);
        self
    }

    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.encoding_type = Some(encoding_type.into());
        self
    }
}

/// Part information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PartInfo {
    /// Part number
    pub part_number: u32,
    /// Part last modified time
    pub last_modified: String,
    /// Part ETag value
    #[serde(rename = "ETag")]
    pub etag: String,
    /// Part size (bytes)
    pub size: u64,
}

/// ListParts response
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListPartsResult {
    /// Bucket name
    pub bucket: String,
    /// Object name
    pub key: String,
    /// Multipart upload ID
    pub upload_id: String,
    /// Starting position returned by this List operation
    pub part_number_marker: u32,
    /// Starting point for the next List operation
    pub next_part_number_marker: u32,
    /// Maximum number of parts returned
    pub max_parts: u32,
    /// Indicates whether all results have been returned
    pub is_truncated: bool,
    /// List of part information
    #[serde(rename = "Part", default)]
    pub parts: Vec<PartInfo>,
    /// Encoding type
    pub encoding_type: Option<String>,
}

/// ListParts operation
pub struct ListParts {
    pub object_key: String,
    pub params: ListPartsParams,
}

impl Ops for ListParts {
    type Response = BodyResponseProcessor<ListPartsResult>;
    type Body = NoneBody;
    type Query = ListPartsParams;

    fn prepare(self) -> Result<Prepared<ListPartsParams>> {
        Ok(Prepared {
            method: Method::GET,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for ListParts operations
pub trait ListPartsOperations {
    /// List uploaded parts
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/listparts>
    fn list_parts(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        params: Option<ListPartsParams>,
    ) -> impl Future<Output = Result<ListPartsResult>>;
}

impl ListPartsOperations for Client {
    async fn list_parts(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        params: Option<ListPartsParams>,
    ) -> Result<ListPartsResult> {
        let final_params = params.unwrap_or_else(|| ListPartsParams::new(upload_id));

        let ops = ListParts {
            object_key: object_key.into(),
            params: final_params,
        };
        self.request(ops).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// ListParts request builder
#[derive(Debug, Clone)]
pub struct ListPartsRequestBuilder {
    upload_id: String,
    max_parts: Option<u32>,
    part_number_marker: Option<u32>,
    encoding_type: Option<String>,
}

impl ListPartsRequestBuilder {
    /// Create a new request builder
    pub fn new(upload_id: impl Into<String>) -> Self {
        Self {
            upload_id: upload_id.into(),
            max_parts: None,
            part_number_marker: None,
            encoding_type: None,
        }
    }

    /// Set maximum number of parts to return
    pub fn max_parts(mut self, max_parts: u32) -> Self {
        self.max_parts = Some(max_parts);
        self
    }

    /// Set part number marker
    pub fn part_number_marker(mut self, part_number_marker: u32) -> Self {
        self.part_number_marker = Some(part_number_marker);
        self
    }

    /// Set encoding type
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.encoding_type = Some(encoding_type.into());
        self
    }

    /// Build request parameters
    pub fn build(self) -> ListPartsParams {
        ListPartsParams {
            upload_id: self.upload_id,
            max_parts: self.max_parts,
            part_number_marker: self.part_number_marker,
            encoding_type: self.encoding_type,
        }
    }
}
