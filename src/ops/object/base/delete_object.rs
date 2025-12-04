use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// DeleteObject request parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteObjectParams {
    /// Version ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl DeleteObjectParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

/// DeleteObject operation
pub struct DeleteObject {
    pub object_key: String,
    pub params: DeleteObjectParams,
}

impl Ops for DeleteObject {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteObjectParams;

    fn prepare(self) -> Result<Prepared<DeleteObjectParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for DeleteObject operations
pub trait DeleteObjectOperations {
    /// Delete an object
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteobject>
    fn delete_object(
        &self,
        object_key: impl Into<String>,
        params: Option<DeleteObjectParams>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteObjectOperations for Client {
    async fn delete_object(
        &self,
        object_key: impl Into<String>,
        params: Option<DeleteObjectParams>,
    ) -> Result<()> {
        let ops = DeleteObject {
            object_key: object_key.into(),
            params: params.unwrap_or_default(),
        };

        self.request(ops).await
    }
}
