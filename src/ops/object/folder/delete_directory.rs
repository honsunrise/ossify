//! DeleteDirectory operation.
//!
//! Deletes a directory in an HNS-enabled bucket. Two modes are supported:
//!
//! * **Non-recursive** (default) — only succeeds if the directory is empty.
//! * **Recursive** — sets `x-oss-delete-recursive: true` and traverses the
//!   subtree. The response may include `<NextDeleteToken>` indicating that
//!   the caller must continue by passing the token in `x-oss-delete-token`.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletedirectory>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// DeleteDirectory query parameters: `?x-oss-delete`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteDirectoryParams {
    #[serde(rename = "x-oss-delete")]
    pub(crate) x_oss_delete: OnlyKeyField,
}

impl DeleteDirectoryParams {
    pub fn new() -> Self {
        Self::default()
    }
}

/// DeleteDirectory request options (HTTP headers).
#[derive(Debug, Clone, Default)]
pub struct DeleteDirectoryOptions {
    /// Whether to recursively delete objects and sub-directories.
    pub recursive: Option<bool>,
    /// Continuation token returned from a previous recursive delete.
    pub delete_token: Option<String>,
}

impl DeleteDirectoryOptions {
    pub fn recursive(mut self, v: bool) -> Self {
        self.recursive = Some(v);
        self
    }

    pub fn delete_token(mut self, token: impl Into<String>) -> Self {
        self.delete_token = Some(token.into());
        self
    }

    fn into_headers(self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        if let Some(v) = self.recursive {
            headers.insert(HeaderName::from_static("x-oss-delete-recursive"), v.to_string().parse()?);
        }
        if let Some(token) = self.delete_token {
            headers.insert(HeaderName::from_static("x-oss-delete-token"), token.parse()?);
        }
        Ok(headers)
    }
}

/// DeleteDirectory response body (`<DeleteDirectoryResult>`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "DeleteDirectoryResult", rename_all = "PascalCase")]
pub struct DeleteDirectoryResponse {
    pub directory_name: String,
    pub delete_number: u64,
    pub next_delete_token: Option<String>,
}

/// DeleteDirectory operation.
pub struct DeleteDirectory {
    pub object_key: String,
    pub params: DeleteDirectoryParams,
    pub options: DeleteDirectoryOptions,
}

impl Ops for DeleteDirectory {
    type Response = BodyResponseProcessor<DeleteDirectoryResponse>;
    type Body = NoneBody;
    type Query = DeleteDirectoryParams;

    fn prepare(self) -> Result<Prepared<DeleteDirectoryParams>> {
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.object_key),
            query: Some(self.params),
            headers: Some(self.options.into_headers()?),
            ..Default::default()
        })
    }
}

/// Trait for DeleteDirectory operations.
pub trait DeleteDirectoryOperations {
    fn delete_directory(
        &self,
        object_key: impl Into<String>,
        options: Option<DeleteDirectoryOptions>,
    ) -> impl Future<Output = Result<DeleteDirectoryResponse>>;
}

impl DeleteDirectoryOperations for Client {
    async fn delete_directory(
        &self,
        object_key: impl Into<String>,
        options: Option<DeleteDirectoryOptions>,
    ) -> Result<DeleteDirectoryResponse> {
        let ops = DeleteDirectory {
            object_key: object_key.into(),
            params: DeleteDirectoryParams::new(),
            options: options.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&DeleteDirectoryParams::new()).unwrap();
        assert_eq!(q, "x-oss-delete");
    }

    #[test]
    fn test_options_recursive_headers() {
        let h = DeleteDirectoryOptions::default()
            .recursive(true)
            .delete_token("TOKEN")
            .into_headers()
            .unwrap();
        assert_eq!(h.get("x-oss-delete-recursive").unwrap(), "true");
        assert_eq!(h.get("x-oss-delete-token").unwrap(), "TOKEN");
    }

    #[test]
    fn test_deserialize_response() {
        let xml = r#"<?xml version="1.0"?>
<DeleteDirectoryResult>
  <DirectoryName>desktop/osstest/a</DirectoryName>
  <DeleteNumber>100</DeleteNumber>
  <NextDeleteToken>Cg9kZXNrdG9wL29zcy9hLzk-</NextDeleteToken>
</DeleteDirectoryResult>"#;
        let resp: DeleteDirectoryResponse = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(resp.directory_name, "desktop/osstest/a");
        assert_eq!(resp.delete_number, 100);
        assert_eq!(resp.next_delete_token.as_deref(), Some("Cg9kZXNrdG9wL29zcy9hLzk-"));
    }

    #[test]
    fn test_deserialize_response_without_next_token() {
        let xml = r#"<DeleteDirectoryResult>
  <DirectoryName>a</DirectoryName>
  <DeleteNumber>1</DeleteNumber>
</DeleteDirectoryResult>"#;
        let resp: DeleteDirectoryResponse = quick_xml::de::from_str(xml).unwrap();
        assert!(resp.next_delete_token.is_none());
    }
}
