use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use super::super::base::StorageClass;
use crate::body::ZeroBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::utils::escape_path;
use crate::{Client, Ops, Prepared, Request};

/// PutSymlink request parameters (query parameters)
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutSymlinkParams {
    symlink: OnlyKeyField,
}

impl PutSymlinkParams {
    pub fn new() -> Self {
        Self {
            symlink: OnlyKeyField,
        }
    }
}

/// PutSymlink request options (mainly set via HTTP headers)
#[derive(Debug, Clone, Default)]
pub struct PutSymlinkOptions {
    /// Whether to forbid overwriting files with the same name
    pub forbid_overwrite: Option<bool>,
    /// Storage class
    pub storage_class: Option<StorageClass>,
    /// Object access control list
    pub object_acl: Option<String>,
}

impl PutSymlinkOptions {
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

    /// Set object ACL
    pub fn object_acl(mut self, acl: impl Into<String>) -> Self {
        self.object_acl = Some(acl.into());
        self
    }
}

impl PutSymlinkOptions {
    fn into_headers(self, target_object: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        let target_object = escape_path(target_object);
        headers.insert(HeaderName::from_static("x-oss-symlink-target"), target_object.parse()?);

        if let Some(forbid_overwrite) = self.forbid_overwrite {
            headers.insert(
                HeaderName::from_static("x-oss-forbid-overwrite"),
                forbid_overwrite.to_string().parse()?,
            );
        }

        if let Some(storage_class) = self.storage_class {
            headers.insert(HeaderName::from_static("x-oss-storage-class"), storage_class.as_ref().parse()?);
        }

        if let Some(object_acl) = self.object_acl {
            headers.insert(HeaderName::from_static("x-oss-object-acl"), object_acl.parse()?);
        }

        Ok(headers)
    }
}

/// PutSymlink response (mainly obtained from response headers)
#[derive(Debug, Clone, Deserialize)]
pub struct PutSymlinkResponse {
    /// ETag value
    #[serde(rename = "etag")]
    pub etag: String,
    /// Version ID (if versioning is enabled)
    #[serde(rename = "x-oss-version-id")]
    pub version_id: Option<String>,
}

/// PutSymlink operation
pub struct PutSymlink {
    pub object_key: String,
    pub target_object: String,
    pub params: PutSymlinkParams,
    pub options: PutSymlinkOptions,
}

impl Ops for PutSymlink {
    type Response = HeaderResponseProcessor<PutSymlinkResponse>;
    type Body = ZeroBody;
    type Query = PutSymlinkParams;

    fn prepare(self) -> Result<Prepared<PutSymlinkParams>> {
        Ok(Prepared {
            method: Method::PUT,
            key: Some(self.object_key),
            query: Some(self.params),
            headers: Some(self.options.into_headers(&self.target_object)?),
            body: Some(()),
            ..Default::default()
        })
    }
}

/// Trait for PutSymlink operations
pub trait PutSymlinkOperations {
    /// Create a symbolic link that points to a destination object
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/putsymlink>
    fn put_symlink(
        &self,
        symlink_key: impl Into<String>,
        target_object: impl Into<String>,
        options: Option<PutSymlinkOptions>,
    ) -> impl Future<Output = Result<PutSymlinkResponse>>;
}

impl PutSymlinkOperations for Client {
    async fn put_symlink(
        &self,
        symlink_key: impl Into<String>,
        target_object: impl Into<String>,
        options: Option<PutSymlinkOptions>,
    ) -> Result<PutSymlinkResponse> {
        let ops = PutSymlink {
            object_key: symlink_key.into(),
            target_object: target_object.into(),
            params: PutSymlinkParams::new(),
            options: options.unwrap_or_default(),
        };

        self.request(ops).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// PutSymlink request builder
#[derive(Debug, Clone, Default)]
pub struct PutSymlinkRequestBuilder {
    options: PutSymlinkOptions,
}

impl PutSymlinkRequestBuilder {
    pub fn new() -> Self {
        Self::default()
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

    /// Set object ACL
    pub fn object_acl(mut self, acl: impl Into<String>) -> Self {
        self.options.object_acl = Some(acl.into());
        self
    }

    /// Build options
    pub fn build(self) -> PutSymlinkOptions {
        self.options
    }
}
