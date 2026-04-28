//! SealAppendObject operation.
//!
//! Stops appending content to an Appendable Object, making it non-appendable.
//! After sealing, the storage class can later be transitioned to Cold Archive
//! or Deep Cold Archive via lifecycle rules.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/sealappendobject>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// SealAppendObject query parameters: `?seal&position=<n>`.
#[derive(Debug, Clone, Serialize)]
pub struct SealAppendObjectParams {
    pub(crate) seal: OnlyKeyField,
    pub position: u64,
}

impl SealAppendObjectParams {
    /// Create new parameters with the expected object length.
    pub fn new(position: u64) -> Self {
        Self {
            seal: OnlyKeyField,
            position,
        }
    }
}

/// SealAppendObject response headers.
#[derive(Debug, Clone, Deserialize)]
pub struct SealAppendObjectResponse {
    /// The time when the object was first sealed (RFC 2822 GMT format).
    #[serde(rename = "x-oss-sealed-time")]
    pub sealed_time: Option<String>,
    /// Object ETag.
    #[serde(rename = "etag")]
    pub etag: Option<String>,
}

/// SealAppendObject operation.
pub struct SealAppendObject {
    pub object_key: String,
    pub params: SealAppendObjectParams,
}

impl Ops for SealAppendObject {
    type Response = HeaderResponseProcessor<SealAppendObjectResponse>;
    type Body = NoneBody;
    type Query = SealAppendObjectParams;

    fn prepare(self) -> Result<Prepared<SealAppendObjectParams>> {
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for SealAppendObject operations.
pub trait SealAppendObjectOperations {
    /// Seal an Appendable Object, preventing further appends.
    fn seal_append_object(
        &self,
        object_key: impl Into<String>,
        position: u64,
    ) -> impl Future<Output = Result<SealAppendObjectResponse>>;
}

impl SealAppendObjectOperations for Client {
    async fn seal_append_object(
        &self,
        object_key: impl Into<String>,
        position: u64,
    ) -> Result<SealAppendObjectResponse> {
        let ops = SealAppendObject {
            object_key: object_key.into(),
            params: SealAppendObjectParams::new(position),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let params = SealAppendObjectParams::new(344606);
        let query = crate::ser::to_string(&params).unwrap();
        assert_eq!(query, "position=344606&seal");
    }

    #[test]
    fn test_deserialize_response() {
        // HeaderResponseProcessor converts headers into a map and then
        // deserializes from JSON, so we simulate the same path here.
        let json = serde_json::json!({
            "x-oss-sealed-time": "Wed, 07 May 2025 23:00:00 GMT",
            "etag": "\"abc\"",
        });
        let resp: SealAppendObjectResponse = serde_json::from_value(json).unwrap();
        assert_eq!(resp.sealed_time.as_deref(), Some("Wed, 07 May 2025 23:00:00 GMT"));
        assert_eq!(resp.etag.as_deref(), Some("\"abc\""));
    }
}
