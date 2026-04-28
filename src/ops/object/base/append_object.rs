//! AppendObject operation.
//!
//! Appends data to an Appendable object. Each append must specify the current
//! object length as the `position`; the server returns the position for the
//! next append in the `x-oss-next-append-position` response header.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/appendobject>

use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;

use bytes::Bytes;
use futures::{TryStream, stream};
use heck::ToKebabCase;
use http::{HeaderMap, HeaderName, Method, header};
use serde::{Deserialize, Serialize};

use crate::body::StreamBody;
use crate::error::Result;
use crate::ops::common::{ServerSideEncryption, StorageClass};
use crate::response::HeaderResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{BoxError, Client, Ops, Prepared, Request, ser};

/// AppendObject query parameters: `?append&position=<n>`.
#[derive(Debug, Clone, Serialize)]
pub struct AppendObjectParams {
    pub(crate) append: OnlyKeyField,
    pub position: u64,
}

impl AppendObjectParams {
    /// Create a new parameters object for the given append position.
    pub fn new(position: u64) -> Self {
        Self {
            append: OnlyKeyField,
            position,
        }
    }
}

/// AppendObject request options (HTTP headers).
///
/// Note: some headers only take effect on the **first** append (where
/// `position == 0`), such as `x-oss-storage-class`, `x-oss-meta-*`, and
/// `x-oss-tagging`.
#[derive(Debug, Clone, Default)]
pub struct AppendObjectOptions {
    pub cache_control: Option<String>,
    pub content_disposition: Option<String>,
    pub content_encoding: Option<String>,
    pub content_type: Option<String>,
    pub expires: Option<String>,
    pub content_md5: Option<String>,
    pub storage_class: Option<StorageClass>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub object_acl: Option<String>,
    pub user_meta: HashMap<String, String>,
    pub tagging: HashMap<String, String>,
}

impl AppendObjectOptions {
    pub fn cache_control(mut self, v: impl Into<String>) -> Self {
        self.cache_control = Some(v.into());
        self
    }

    pub fn content_disposition(mut self, v: impl Into<String>) -> Self {
        self.content_disposition = Some(v.into());
        self
    }

    pub fn content_encoding(mut self, v: impl Into<String>) -> Self {
        self.content_encoding = Some(v.into());
        self
    }

    pub fn content_type(mut self, v: impl Into<String>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    pub fn expires(mut self, v: impl Into<String>) -> Self {
        self.expires = Some(v.into());
        self
    }

    pub fn content_md5(mut self, v: impl Into<String>) -> Self {
        self.content_md5 = Some(v.into());
        self
    }

    pub fn storage_class(mut self, v: StorageClass) -> Self {
        self.storage_class = Some(v);
        self
    }

    pub fn server_side_encryption(mut self, v: ServerSideEncryption) -> Self {
        self.server_side_encryption = Some(v);
        self
    }

    pub fn object_acl(mut self, v: impl Into<String>) -> Self {
        self.object_acl = Some(v.into());
        self
    }

    pub fn user_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.user_meta.insert(key.into(), value.into());
        self
    }

    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tagging.insert(key.into(), value.into());
        self
    }

    fn into_headers(self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        if let Some(v) = self.cache_control {
            headers.insert(header::CACHE_CONTROL, v.parse()?);
        }
        if let Some(v) = self.content_disposition {
            headers.insert(header::CONTENT_DISPOSITION, v.parse()?);
        }
        if let Some(v) = self.content_encoding {
            headers.insert(header::CONTENT_ENCODING, v.parse()?);
        }
        if let Some(v) = self.content_type {
            headers.insert(header::CONTENT_TYPE, v.parse()?);
        }
        if let Some(v) = self.expires {
            headers.insert(header::EXPIRES, v.parse()?);
        }
        if let Some(v) = self.content_md5 {
            headers.insert(HeaderName::from_static("content-md5"), v.parse()?);
        }
        if let Some(v) = self.storage_class {
            headers.insert(HeaderName::from_static("x-oss-storage-class"), v.as_ref().parse()?);
        }
        if let Some(v) = self.server_side_encryption {
            headers.insert(HeaderName::from_static("x-oss-server-side-encryption"), v.as_ref().parse()?);
        }
        if let Some(v) = self.object_acl {
            headers.insert(HeaderName::from_static("x-oss-object-acl"), v.parse()?);
        }
        for (key, value) in self.user_meta {
            let key = key.to_kebab_case().to_lowercase();
            let header_name = format!("x-oss-meta-{key}");
            headers.insert(HeaderName::from_bytes(header_name.as_bytes())?, value.parse()?);
        }
        if !self.tagging.is_empty() {
            let tagging_str = ser::to_string(&self.tagging)?;
            headers.insert(HeaderName::from_static("x-oss-tagging"), tagging_str.parse()?);
        }

        Ok(headers)
    }
}

/// AppendObject response (fields from headers).
#[derive(Debug, Clone, Deserialize)]
pub struct AppendObjectResponse {
    /// ETag value.
    #[serde(rename = "etag")]
    pub etag: Option<String>,
    /// Position (in bytes) for the next append.
    #[serde(rename = "x-oss-next-append-position")]
    pub next_append_position: Option<u64>,
    /// CRC64 of the entire object.
    #[serde(rename = "x-oss-hash-crc64ecma")]
    pub hash_crc64ecma: Option<String>,
    /// Version ID (versioning-enabled bucket).
    #[serde(rename = "x-oss-version-id")]
    pub version_id: Option<String>,
    /// Server-side encryption.
    #[serde(rename = "x-oss-server-side-encryption")]
    pub server_side_encryption: Option<String>,
}

/// AppendObject operation.
pub struct AppendObject<S> {
    pub object_key: String,
    pub params: AppendObjectParams,
    pub options: AppendObjectOptions,
    pub stream_body: S,
}

impl<S> Ops for AppendObject<S>
where
    S: TryStream + Send + 'static,
    S::Error: Into<BoxError>,
    Bytes: From<S::Ok>,
{
    type Response = HeaderResponseProcessor<AppendObjectResponse>;
    type Body = StreamBody<S>;
    type Query = AppendObjectParams;

    fn prepare(self) -> Result<Prepared<AppendObjectParams, S>> {
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.object_key),
            query: Some(self.params),
            headers: Some(self.options.into_headers()?),
            body: Some(self.stream_body),
            ..Default::default()
        })
    }
}

/// Trait for AppendObject operations.
pub trait AppendObjectOperations {
    /// Append data to an Appendable object.
    fn append_object<T>(
        &self,
        object_key: impl Into<String>,
        position: u64,
        body: T,
        options: Option<AppendObjectOptions>,
    ) -> impl Future<Output = Result<AppendObjectResponse>>
    where
        T: Send + 'static,
        Bytes: From<T>;

    /// Append streaming data to an Appendable object.
    fn append_object_stream<S>(
        &self,
        object_key: impl Into<String>,
        position: u64,
        body: S,
        options: Option<AppendObjectOptions>,
    ) -> impl Future<Output = Result<AppendObjectResponse>>
    where
        S: TryStream + Send + 'static,
        S::Error: Into<BoxError>,
        Bytes: From<S::Ok>;
}

impl AppendObjectOperations for Client {
    async fn append_object<T>(
        &self,
        object_key: impl Into<String>,
        position: u64,
        body: T,
        options: Option<AppendObjectOptions>,
    ) -> Result<AppendObjectResponse>
    where
        T: Send + 'static,
        Bytes: From<T>,
    {
        let ops = AppendObject {
            object_key: object_key.into(),
            params: AppendObjectParams::new(position),
            options: options.unwrap_or_default(),
            stream_body: stream::once(async move { Result::<Bytes, Infallible>::Ok(body.into()) }),
        };
        self.request(ops).await
    }

    async fn append_object_stream<S>(
        &self,
        object_key: impl Into<String>,
        position: u64,
        body: S,
        options: Option<AppendObjectOptions>,
    ) -> Result<AppendObjectResponse>
    where
        S: TryStream + Send + 'static,
        S::Error: Into<BoxError>,
        Bytes: From<S::Ok>,
    {
        let ops = AppendObject {
            object_key: object_key.into(),
            params: AppendObjectParams::new(position),
            options: options.unwrap_or_default(),
            stream_body: body,
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let params = AppendObjectParams::new(65536);
        let query = crate::ser::to_string(&params).unwrap();
        assert_eq!(query, "append&position=65536");
    }

    #[test]
    fn test_serialize_params_position_zero() {
        let params = AppendObjectParams::new(0);
        let query = crate::ser::to_string(&params).unwrap();
        assert_eq!(query, "append&position=0");
    }

    #[test]
    fn test_options_headers_roundtrip() {
        let opts = AppendObjectOptions::default()
            .cache_control("no-cache")
            .content_type("image/jpeg")
            .storage_class(StorageClass::Archive)
            .user_meta("Author", "alice")
            .tag("TagA", "A");
        let headers = opts.into_headers().unwrap();
        assert_eq!(headers.get(header::CACHE_CONTROL).unwrap(), "no-cache");
        assert_eq!(headers.get(header::CONTENT_TYPE).unwrap(), "image/jpeg");
        assert_eq!(headers.get("x-oss-storage-class").unwrap(), "Archive");
        assert_eq!(headers.get("x-oss-meta-author").unwrap(), "alice");
        assert!(
            headers
                .get("x-oss-tagging")
                .unwrap()
                .to_str()
                .unwrap()
                .contains("TagA=A")
        );
    }
}
