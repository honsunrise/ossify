//! CreateDirectory operation.
//!
//! Creates a directory in an HNS-enabled bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createdirectory>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// CreateDirectory query parameters: `?x-oss-dir`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateDirectoryParams {
    #[serde(rename = "x-oss-dir")]
    pub(crate) x_oss_dir: OnlyKeyField,
}

impl CreateDirectoryParams {
    pub fn new() -> Self {
        Self::default()
    }
}

/// CreateDirectory operation.
pub struct CreateDirectory {
    pub object_key: String,
    pub params: CreateDirectoryParams,
}

impl Ops for CreateDirectory {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = CreateDirectoryParams;

    fn prepare(self) -> Result<Prepared<CreateDirectoryParams>> {
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for CreateDirectory operations.
pub trait CreateDirectoryOperations {
    /// Create a directory (HNS-enabled buckets only).
    fn create_directory(&self, object_key: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl CreateDirectoryOperations for Client {
    async fn create_directory(&self, object_key: impl Into<String>) -> Result<()> {
        let ops = CreateDirectory {
            object_key: object_key.into(),
            params: CreateDirectoryParams::new(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&CreateDirectoryParams::new()).unwrap();
        assert_eq!(q, "x-oss-dir");
    }

    #[test]
    fn test_prepare_method_and_key() {
        let p = CreateDirectory {
            object_key: "desktop/oss".into(),
            params: CreateDirectoryParams::new(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::POST);
        assert_eq!(p.key.as_deref(), Some("desktop/oss"));
    }
}
