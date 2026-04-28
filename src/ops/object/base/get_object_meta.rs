//! GetObjectMeta operation.
//!
//! Returns the basic metadata (ETag, size, last-modified time) of an object
//! without downloading the object content. Unlike `HeadObject`, only a limited
//! subset of headers is returned.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getobjectmeta>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// GetObjectMeta query parameters.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectMetaParams {
    pub(crate) object_meta: OnlyKeyField,
    /// Optional target version ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetObjectMetaParams {
    pub fn new() -> Self {
        Self {
            object_meta: OnlyKeyField,
            version_id: None,
        }
    }

    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

impl Default for GetObjectMetaParams {
    fn default() -> Self {
        Self::new()
    }
}

/// GetObjectMeta response headers.
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectMetaResponse {
    /// Size of the object in bytes (`Content-Length`).
    #[serde(rename = "content-length")]
    pub content_length: Option<String>,
    /// ETag.
    #[serde(rename = "etag")]
    pub etag: Option<String>,
    /// Last-modified time.
    #[serde(rename = "last-modified")]
    pub last_modified: Option<String>,
    /// Version ID (versioned buckets).
    #[serde(rename = "x-oss-version-id")]
    pub version_id: Option<String>,
    /// Last access time (when access tracking is enabled).
    #[serde(rename = "x-oss-last-access-time")]
    pub last_access_time: Option<String>,
    /// Transition time to Cold/Deep Cold Archive.
    #[serde(rename = "x-oss-transition-time")]
    pub transition_time: Option<String>,
    /// Sealed time (sealed Appendable objects).
    #[serde(rename = "x-oss-sealed-time")]
    pub sealed_time: Option<String>,
}

/// GetObjectMeta operation.
pub struct GetObjectMeta {
    pub object_key: String,
    pub params: GetObjectMetaParams,
}

impl Ops for GetObjectMeta {
    type Response = HeaderResponseProcessor<GetObjectMetaResponse>;
    type Body = NoneBody;
    type Query = GetObjectMetaParams;

    fn prepare(self) -> Result<Prepared<GetObjectMetaParams>> {
        Ok(Prepared {
            method: Method::HEAD,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for GetObjectMeta operations.
pub trait GetObjectMetaOperations {
    fn get_object_meta(
        &self,
        object_key: impl Into<String>,
        params: Option<GetObjectMetaParams>,
    ) -> impl Future<Output = Result<GetObjectMetaResponse>>;
}

impl GetObjectMetaOperations for Client {
    async fn get_object_meta(
        &self,
        object_key: impl Into<String>,
        params: Option<GetObjectMetaParams>,
    ) -> Result<GetObjectMetaResponse> {
        let ops = GetObjectMeta {
            object_key: object_key.into(),
            params: params.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params_default() {
        let q = crate::ser::to_string(&GetObjectMetaParams::new()).unwrap();
        assert_eq!(q, "objectMeta");
    }

    #[test]
    fn test_serialize_params_with_version() {
        let q = crate::ser::to_string(&GetObjectMetaParams::new().version_id("v1")).unwrap();
        assert_eq!(q, "objectMeta&versionId=v1");
    }

    #[test]
    fn test_deserialize_response() {
        let json = serde_json::json!({
            "content-length": "344606",
            "etag": "\"5B3C1A2E\"",
            "last-modified": "Fri, 24 Feb 2012 06:07:48 GMT",
            "x-oss-version-id": "v1",
        });
        let resp: GetObjectMetaResponse = serde_json::from_value(json).unwrap();
        assert_eq!(resp.content_length.as_deref(), Some("344606"));
        assert_eq!(resp.version_id.as_deref(), Some("v1"));
    }
}
