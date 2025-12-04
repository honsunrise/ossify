use std::future::Future;

use http::Method;
use serde::{Deserialize, Deserializer, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// ListMultipartUploads request parameters
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ListMultipartUploadsParams {
    uploads: OnlyKeyField,

    /// Character used to group file names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// Limit returned file keys to those with prefix as a prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Multipart upload events where all file names are lexicographically greater than the key-marker parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    /// Used together with key-marker parameter to specify the starting position of returned results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id_marker: Option<String>,
    /// Limit the maximum number of multipart upload events returned, default and maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploads: Option<u32>,
    /// Specify encoding for returned keys, currently supports URL encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

impl ListMultipartUploadsParams {
    pub fn new() -> Self {
        Self {
            uploads: OnlyKeyField,
            ..Default::default()
        }
    }

    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn key_marker(mut self, key_marker: impl Into<String>) -> Self {
        self.key_marker = Some(key_marker.into());
        self
    }

    pub fn upload_id_marker(mut self, upload_id_marker: impl Into<String>) -> Self {
        self.upload_id_marker = Some(upload_id_marker.into());
        self
    }

    pub fn max_uploads(mut self, max_uploads: u32) -> Self {
        self.max_uploads = Some(max_uploads);
        self
    }

    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.encoding_type = Some(encoding_type.into());
        self
    }
}

/// Owner information of multipart upload event
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,
    pub display_name: String,
}

/// Multipart upload event information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultipartUpload {
    /// Object name
    pub key: String,
    /// Multipart upload ID
    pub upload_id: String,
    /// Initialization time of multipart upload event
    pub initiated: String,
    /// Storage class
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    /// Owner information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}

/// Common prefix information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonPrefix {
    pub prefix: String,
}

fn unwrap_uploads<'de, D>(deserializer: D) -> std::result::Result<Vec<MultipartUpload>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Uploads {
        #[serde(default)]
        upload: Vec<MultipartUpload>,
    }
    Ok(Uploads::deserialize(deserializer)?.upload)
}

fn unwrap_common_prefixes<'de, D>(deserializer: D) -> std::result::Result<Vec<CommonPrefix>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct CommonPrefixes {
        #[serde(default)]
        prefix: Vec<CommonPrefix>,
    }
    Ok(CommonPrefixes::deserialize(deserializer)?.prefix)
}

/// ListMultipartUploads response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMultipartUploadsResult {
    /// Bucket name
    pub bucket: String,
    /// Prefix for this query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Character used to group object names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// Starting point for this List Multipart Upload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    /// Used together with KeyMarker parameter to specify the starting position of returned results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id_marker: Option<String>,
    /// Starting point for the next List Multipart Upload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key_marker: Option<String>,
    /// Starting point for the next List Multipart Upload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_upload_id_marker: Option<String>,
    /// Maximum number of uploads returned
    pub max_uploads: u32,
    /// Indicates whether all results have been returned
    pub is_truncated: bool,
    /// List of multipart upload events
    #[serde(default, deserialize_with = "unwrap_uploads")]
    pub uploads: Vec<MultipartUpload>,
    /// If delimiter is specified in the request, returns a collection of paths with the specified prefix, ending with delimiter, and having common prefixes
    #[serde(default, deserialize_with = "unwrap_common_prefixes")]
    pub common_prefixes: Vec<CommonPrefix>,
    /// Encoding type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

/// ListMultipartUploads operation
pub struct ListMultipartUploads {
    pub params: ListMultipartUploadsParams,
}

impl Ops for ListMultipartUploads {
    type Response = BodyResponseProcessor<ListMultipartUploadsResult>;
    type Body = NoneBody;
    type Query = ListMultipartUploadsParams;

    fn prepare(self) -> Result<Prepared<ListMultipartUploadsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for ListMultipartUploads operations
pub trait ListMultipartUploadsOperations {
    /// List multipart upload events
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/listmultipartuploads>
    fn list_multipart_uploads(
        &self,
        params: Option<ListMultipartUploadsParams>,
    ) -> impl Future<Output = Result<ListMultipartUploadsResult>>;
}

impl ListMultipartUploadsOperations for Client {
    async fn list_multipart_uploads(
        &self,
        params: Option<ListMultipartUploadsParams>,
    ) -> Result<ListMultipartUploadsResult> {
        let ops = ListMultipartUploads {
            params: params.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// ListMultipartUploads request builder
#[derive(Debug, Clone, Default)]
pub struct ListMultipartUploadsRequestBuilder {
    params: ListMultipartUploadsParams,
}

impl ListMultipartUploadsRequestBuilder {
    /// Create a new request builder
    pub fn new() -> Self {
        Self {
            params: ListMultipartUploadsParams::new(),
        }
    }

    /// Set delimiter
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.params = self.params.delimiter(delimiter);
        self
    }

    /// Set prefix
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.params = self.params.prefix(prefix);
        self
    }

    /// Set key marker
    pub fn key_marker(mut self, key_marker: impl Into<String>) -> Self {
        self.params = self.params.key_marker(key_marker);
        self
    }

    /// Set upload ID marker
    pub fn upload_id_marker(mut self, upload_id_marker: impl Into<String>) -> Self {
        self.params = self.params.upload_id_marker(upload_id_marker);
        self
    }

    /// Set maximum number of returns
    pub fn max_uploads(mut self, max_uploads: u32) -> Self {
        self.params = self.params.max_uploads(max_uploads);
        self
    }

    /// Set encoding type
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.params = self.params.encoding_type(encoding_type);
        self
    }

    /// Build request parameters
    pub fn build(self) -> ListMultipartUploadsParams {
        self.params
    }
}
