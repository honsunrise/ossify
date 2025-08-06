use std::borrow::Cow;
use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::EmptyBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Request};

/// AbortMultipartUpload request parameters
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbortMultipartUploadParams {
    pub upload_id: String,
}

impl AbortMultipartUploadParams {
    pub fn new(upload_id: impl Into<String>) -> Self {
        Self {
            upload_id: upload_id.into(),
        }
    }
}

/// AbortMultipartUpload operation
pub struct AbortMultipartUpload {
    pub object_key: String,
    pub params: AbortMultipartUploadParams,
}

impl Ops for AbortMultipartUpload {
    type Response = EmptyResponseProcessor;
    type Body = EmptyBody;
    type Query = AbortMultipartUploadParams;

    const PRODUCT: &'static str = "oss";

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn key<'a>(&'a self) -> Option<Cow<'a, str>> {
        Some(Cow::Borrowed(&self.object_key))
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.params)
    }
}

/// Trait for AbortMultipartUpload operations
pub trait AbortMultipartUploadOperations {
    /// Cancel multipart upload
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/abortmultipartupload>
    fn abort_multipart_upload(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
    ) -> impl Future<Output = Result<()>>;
}

impl AbortMultipartUploadOperations for Client {
    async fn abort_multipart_upload(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
    ) -> Result<()> {
        let ops = AbortMultipartUpload {
            object_key: object_key.as_ref().to_string(),
            params: AbortMultipartUploadParams::new(upload_id.as_ref()),
        };
        self.request(ops).await
    }
}
