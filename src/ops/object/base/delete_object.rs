use std::borrow::Cow;
use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::EmptyBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Request};

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
    type Body = EmptyBody;
    type Query = DeleteObjectParams;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn key<'a>(&'a self) -> Option<Cow<'a, str>> {
        Some(Cow::Borrowed(&self.object_key))
    }

    fn query(&self) -> Option<&Self::Query> {
        if self.params.version_id.is_some() {
            Some(&self.params)
        } else {
            None
        }
    }
}

/// Trait for DeleteObject operations
pub trait DeleteObjectOperations {
    /// Delete an object
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteobject>
    fn delete_object(
        &self,
        object_key: impl AsRef<str>,
        params: Option<DeleteObjectParams>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteObjectOperations for Client {
    async fn delete_object(
        &self,
        object_key: impl AsRef<str>,
        params: Option<DeleteObjectParams>,
    ) -> Result<()> {
        let ops = DeleteObject {
            object_key: object_key.as_ref().to_string(),
            params: params.unwrap_or_default(),
        };

        self.request(ops).await
    }
}
