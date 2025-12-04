use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

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
    type Body = NoneBody;
    type Query = AbortMultipartUploadParams;

    fn prepare(self) -> Result<Prepared<AbortMultipartUploadParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for AbortMultipartUpload operations
pub trait AbortMultipartUploadOperations {
    /// Cancel multipart upload
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/abortmultipartupload>
    fn abort_multipart_upload(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl AbortMultipartUploadOperations for Client {
    async fn abort_multipart_upload(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
    ) -> Result<()> {
        let ops = AbortMultipartUpload {
            object_key: object_key.into(),
            params: AbortMultipartUploadParams::new(upload_id),
        };
        self.request(ops).await
    }
}
