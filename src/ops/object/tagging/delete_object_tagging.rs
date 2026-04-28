//! DeleteObjectTagging: remove all tags from an object.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteobjecttagging>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// DeleteObjectTagging query parameters.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteObjectTaggingParams {
    pub(crate) tagging: OnlyKeyField,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl DeleteObjectTaggingParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version_id(mut self, v: impl Into<String>) -> Self {
        self.version_id = Some(v.into());
        self
    }
}

/// DeleteObjectTagging operation.
pub struct DeleteObjectTagging {
    pub object_key: String,
    pub params: DeleteObjectTaggingParams,
}

impl Ops for DeleteObjectTagging {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteObjectTaggingParams;

    fn prepare(self) -> Result<Prepared<DeleteObjectTaggingParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for DeleteObjectTagging operations.
pub trait DeleteObjectTaggingOperations {
    fn delete_object_tagging(
        &self,
        object_key: impl Into<String>,
        params: Option<DeleteObjectTaggingParams>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteObjectTaggingOperations for Client {
    async fn delete_object_tagging(
        &self,
        object_key: impl Into<String>,
        params: Option<DeleteObjectTaggingParams>,
    ) -> Result<()> {
        let ops = DeleteObjectTagging {
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
    fn test_serialize_params() {
        let q = crate::ser::to_string(&DeleteObjectTaggingParams::default()).unwrap();
        assert_eq!(q, "tagging");
    }

    #[test]
    fn test_prepare_method() {
        let p = DeleteObjectTagging {
            object_key: "foo.jpg".into(),
            params: DeleteObjectTaggingParams::new().version_id("v1"),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::DELETE);
        assert_eq!(p.key.as_deref(), Some("foo.jpg"));
    }
}
