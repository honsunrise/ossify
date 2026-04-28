//! DeleteMultipleObjects operation.
//!
//! Deletes up to 1000 objects in a single request via `POST /?delete` with an
//! XML body describing the keys (and optional version IDs) to delete.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletemultipleobjects>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// DeleteMultipleObjects query parameters (`?delete`).
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteMultipleObjectsParams {
    pub(crate) delete: OnlyKeyField,
}

impl DeleteMultipleObjectsParams {
    pub fn new() -> Self {
        Self::default()
    }
}

/// A single object to delete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "Object", rename_all = "PascalCase")]
pub struct DeleteObjectEntry {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl DeleteObjectEntry {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: None,
        }
    }

    pub fn with_version(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

/// DeleteMultipleObjects request body (`<Delete>` element).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Delete", rename_all = "PascalCase")]
pub struct DeleteMultipleObjectsConfiguration {
    /// Enable quiet mode (no per-object response body).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet: Option<bool>,
    /// The list of objects to delete.
    #[serde(rename = "Object", default)]
    pub objects: Vec<DeleteObjectEntry>,
}

impl DeleteMultipleObjectsConfiguration {
    pub fn new(objects: Vec<DeleteObjectEntry>) -> Self {
        Self { quiet: None, objects }
    }

    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = Some(quiet);
        self
    }
}

/// A single `<Deleted>` element in the response.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename = "Deleted", rename_all = "PascalCase")]
pub struct DeletedObject {
    pub key: String,
    pub version_id: Option<String>,
    pub delete_marker: Option<bool>,
    pub delete_marker_version_id: Option<String>,
}

/// DeleteMultipleObjects response body (`<DeleteResult>` element).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "DeleteResult", rename_all = "PascalCase")]
pub struct DeleteMultipleObjectsResponse {
    #[serde(rename = "Deleted", default)]
    pub deleted: Vec<DeletedObject>,
    pub encoding_type: Option<String>,
}

/// DeleteMultipleObjects operation.
pub struct DeleteMultipleObjects {
    pub params: DeleteMultipleObjectsParams,
    pub body: DeleteMultipleObjectsConfiguration,
    pub encoding_type: Option<String>,
}

impl Ops for DeleteMultipleObjects {
    type Response = BodyResponseProcessor<DeleteMultipleObjectsResponse>;
    type Body = XMLBody<DeleteMultipleObjectsConfiguration>;
    type Query = DeleteMultipleObjectsParams;

    fn prepare(self) -> Result<Prepared<DeleteMultipleObjectsParams, DeleteMultipleObjectsConfiguration>> {
        let mut headers = HeaderMap::new();
        if let Some(enc) = self.encoding_type {
            headers.insert(HeaderName::from_static("encoding-type"), enc.parse()?);
        }
        Ok(Prepared {
            method: Method::POST,
            key: None,
            query: Some(self.params),
            headers: Some(headers),
            body: Some(self.body),
            ..Default::default()
        })
    }
}

/// Trait for DeleteMultipleObjects operations.
pub trait DeleteMultipleObjectsOperations {
    /// Delete multiple objects in a single request.
    fn delete_multiple_objects(
        &self,
        configuration: DeleteMultipleObjectsConfiguration,
        encoding_type: Option<String>,
    ) -> impl Future<Output = Result<DeleteMultipleObjectsResponse>>;
}

impl DeleteMultipleObjectsOperations for Client {
    async fn delete_multiple_objects(
        &self,
        configuration: DeleteMultipleObjectsConfiguration,
        encoding_type: Option<String>,
    ) -> Result<DeleteMultipleObjectsResponse> {
        let ops = DeleteMultipleObjects {
            params: DeleteMultipleObjectsParams::new(),
            body: configuration,
            encoding_type,
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&DeleteMultipleObjectsParams::new()).unwrap();
        assert_eq!(q, "delete");
    }

    #[test]
    fn test_serialize_body_verbose() {
        let cfg = DeleteMultipleObjectsConfiguration::new(vec![
            DeleteObjectEntry::new("a.txt"),
            DeleteObjectEntry::new("b.txt").with_version("v1"),
        ])
        .quiet(false);

        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<Quiet>false</Quiet>"));
        assert!(xml.contains("<Object><Key>a.txt</Key></Object>"));
        assert!(xml.contains("<Object><Key>b.txt</Key><VersionId>v1</VersionId></Object>"));
    }

    #[test]
    fn test_serialize_body_omits_quiet_when_none() {
        let cfg = DeleteMultipleObjectsConfiguration::new(vec![DeleteObjectEntry::new("a.txt")]);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(!xml.contains("<Quiet>"));
    }

    #[test]
    fn test_deserialize_response_verbose() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<DeleteResult>
  <Deleted><Key>a.txt</Key></Deleted>
  <Deleted>
    <Key>b.txt</Key>
    <VersionId>v1</VersionId>
    <DeleteMarker>true</DeleteMarker>
    <DeleteMarkerVersionId>vm1</DeleteMarkerVersionId>
  </Deleted>
</DeleteResult>"#;
        let resp: DeleteMultipleObjectsResponse = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(resp.deleted.len(), 2);
        assert_eq!(resp.deleted[0].key, "a.txt");
        assert_eq!(resp.deleted[1].version_id.as_deref(), Some("v1"));
        assert_eq!(resp.deleted[1].delete_marker, Some(true));
    }
}
