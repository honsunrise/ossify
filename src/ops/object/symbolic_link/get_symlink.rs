use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// GetSymlink request parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSymlinkParams {
    symlink: OnlyKeyField,

    /// Version ID for retrieving a specific version of the symbolic link
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetSymlinkParams {
    pub fn new() -> Self {
        Self {
            symlink: OnlyKeyField,
            version_id: None,
        }
    }

    /// Set the Version ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

/// GetSymlink response
#[derive(Debug, Clone, Deserialize)]
pub struct GetSymlinkResponse {
    /// The destination object to which the symbolic link points
    #[serde(rename = "x-oss-symlink-target")]
    pub symlink_target: String,
    /// Version ID of the symbolic link (if versioning is enabled)
    #[serde(rename = "x-oss-version-id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Whether this is a delete marker
    #[serde(rename = "x-oss-delete-marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_marker: Option<String>,
}

/// GetSymlink operation
pub struct GetSymlink {
    pub object_key: String,
    pub params: GetSymlinkParams,
}

impl Ops for GetSymlink {
    type Response = HeaderResponseProcessor<GetSymlinkResponse>;
    type Body = NoneBody;
    type Query = GetSymlinkParams;

    fn prepare(self) -> Result<Prepared<GetSymlinkParams>> {
        Ok(Prepared {
            method: Method::GET,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// GetSymlink operations trait
pub trait GetSymlinkOperations {
    /// Get a symbolic link information
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/getsymlink>
    fn get_symlink(
        &self,
        object_key: impl Into<String>,
        params: GetSymlinkParams,
    ) -> impl Future<Output = Result<GetSymlinkResponse>>;
}

impl GetSymlinkOperations for Client {
    async fn get_symlink(
        &self,
        object_key: impl Into<String>,
        params: GetSymlinkParams,
    ) -> Result<GetSymlinkResponse> {
        let ops = GetSymlink {
            object_key: object_key.into(),
            params,
        };

        self.request(ops).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// GetSymlinkRequest builder
#[derive(Debug, Clone, Default)]
pub struct GetSymlinkRequestBuilder {
    params: GetSymlinkParams,
}

impl GetSymlinkRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Version ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.params.version_id = Some(version_id.into());
        self
    }

    /// Build parameters
    pub fn build(self) -> GetSymlinkParams {
        self.params
    }
}
