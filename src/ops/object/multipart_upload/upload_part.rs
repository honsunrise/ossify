use std::borrow::Cow;
use std::future::Future;

use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::BinaryBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::{Client, Ops, Request};

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
pub struct UploadPart {
    pub object_key: String,
    pub params: UploadPartParams,
    pub body: Bytes,
}

impl Ops for UploadPart {
    type Response = HeaderResponseProcessor<UploadPartResult>;
    type Body = BinaryBody;
    type Query = UploadPartParams;

    const PRODUCT: &'static str = "oss";

    fn method(&self) -> Method {
        Method::PUT
    }

    fn key<'a>(&'a self) -> Option<Cow<'a, str>> {
        Some(Cow::Borrowed(&self.object_key))
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.params)
    }

    fn body(&self) -> Option<&Bytes> {
        Some(&self.body)
    }
}

/// Trait for UploadPart operations
pub trait UploadPartOperations {
    /// Upload part
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/uploadpart>
    fn upload_part(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        part_number: u32,
        data: &[u8],
    ) -> impl Future<Output = Result<UploadPartResult>>;
}

impl UploadPartOperations for Client {
    async fn upload_part(
        &self,
        object_key: impl AsRef<str>,
        upload_id: impl AsRef<str>,
        part_number: u32,
        data: &[u8],
    ) -> Result<UploadPartResult> {
        let ops = UploadPart {
            object_key: object_key.as_ref().to_string(),
            params: UploadPartParams::new(part_number, upload_id.as_ref()),
            body: Bytes::copy_from_slice(data),
        };
        self.request(ops).await
    }
}
