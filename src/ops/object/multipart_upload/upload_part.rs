use std::convert::Infallible;
use std::future::Future;

use bytes::Bytes;
use futures::{TryStream, stream};
use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::StreamBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::{BoxError, Client, Ops, Prepared, Request};

/// UploadPart request parameters
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadPartParams {
    pub part_number: u32,
    pub upload_id: String,
}

impl UploadPartParams {
    pub fn new(part_number: u32, upload_id: impl Into<String>) -> Self {
        Self {
            part_number,
            upload_id: upload_id.into(),
        }
    }
}

/// UploadPart response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UploadPartResult {
    pub etag: String,
    #[serde(rename = "x-oss-hash-crc64ecma")]
    pub hash_crc64ecma: String,
    pub content_md5: String,
}

/// UploadPart operation
pub struct UploadPart<S> {
    pub object_key: String,
    pub params: UploadPartParams,
    pub stream_body: S,
}

impl<S> Ops for UploadPart<S>
where
    S: TryStream + Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    Bytes: From<S::Ok>,
{
    type Response = HeaderResponseProcessor<UploadPartResult>;
    type Body = StreamBody<S>;
    type Query = UploadPartParams;

    fn prepare(self) -> Result<Prepared<UploadPartParams, S>> {
        Ok(Prepared {
            method: Method::PUT,
            key: Some(self.object_key),
            query: Some(self.params),
            body: Some(self.stream_body),
            ..Default::default()
        })
    }
}

/// Trait for UploadPart operations
pub trait UploadPartOperations {
    /// Upload part
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/uploadpart>
    fn upload_part<T>(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        part_number: u32,
        body: T,
    ) -> impl Future<Output = Result<UploadPartResult>>
    where
        T: Send + 'static,
        Bytes: From<T>;

    /// Upload part
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/uploadpart>
    fn upload_part_stream<S>(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        part_number: u32,
        stream: S,
    ) -> impl Future<Output = Result<UploadPartResult>>
    where
        S: TryStream + Send + 'static,
        S::Error: Into<BoxError>,
        Bytes: From<S::Ok>;
}

impl UploadPartOperations for Client {
    async fn upload_part<T>(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        part_number: u32,
        body: T,
    ) -> Result<UploadPartResult>
    where
        T: Send + 'static,
        Bytes: From<T>,
    {
        let ops = UploadPart {
            object_key: object_key.into(),
            params: UploadPartParams::new(part_number, upload_id),
            stream_body: stream::once(async move { Result::<Bytes, Infallible>::Ok(body.into()) }),
        };
        self.request(ops).await
    }

    async fn upload_part_stream<S>(
        &self,
        object_key: impl Into<String>,
        upload_id: impl Into<String>,
        part_number: u32,
        stream: S,
    ) -> Result<UploadPartResult>
    where
        S: TryStream + Send + 'static,
        S::Error: Into<BoxError>,
        Bytes: From<S::Ok>,
    {
        let ops = UploadPart {
            object_key: object_key.into(),
            params: UploadPartParams::new(part_number, upload_id),
            stream_body: stream,
        };
        self.request(ops).await
    }
}
