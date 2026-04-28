//! Rename operation.
//!
//! Renames a directory or an object in an HNS-enabled bucket. The source path
//! is passed in the `x-oss-rename-source` header; the destination is the
//! request path.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/rename>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Rename query parameters: `?x-oss-rename`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RenameParams {
    #[serde(rename = "x-oss-rename")]
    pub(crate) x_oss_rename: OnlyKeyField,
}

impl RenameParams {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Rename operation.
pub struct Rename {
    /// Destination key.
    pub destination_key: String,
    /// Source path (passed via `x-oss-rename-source`).
    pub source: String,
    pub params: RenameParams,
}

impl Ops for Rename {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = RenameParams;

    fn prepare(self) -> Result<Prepared<RenameParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-rename-source"), self.source.parse()?);
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.destination_key),
            query: Some(self.params),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

/// Trait for Rename operations.
pub trait RenameOperations {
    /// Rename a directory or object (HNS-enabled buckets only).
    fn rename(
        &self,
        destination_key: impl Into<String>,
        source: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl RenameOperations for Client {
    async fn rename(&self, destination_key: impl Into<String>, source: impl Into<String>) -> Result<()> {
        let ops = Rename {
            destination_key: destination_key.into(),
            source: source.into(),
            params: RenameParams::new(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&RenameParams::new()).unwrap();
        assert_eq!(q, "x-oss-rename");
    }

    #[test]
    fn test_prepare_sets_source_header() {
        let p = Rename {
            destination_key: "desktop/osstest/b".into(),
            source: "desktop/osstest/a".into(),
            params: RenameParams::new(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::POST);
        assert_eq!(p.key.as_deref(), Some("desktop/osstest/b"));
        assert_eq!(p.headers.unwrap().get("x-oss-rename-source").unwrap(), "desktop/osstest/a");
    }
}
