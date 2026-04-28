//! ListVectorIndexes: list indexes in the current vector bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listvectorindexes>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::JSONBody;
use crate::error::Result;
use crate::ops::common::VectorIndexInfo;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListVectorIndexesParams {
    #[serde(rename = "listVectorIndexes")]
    list_vector_indexes: OnlyKeyField,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListVectorIndexesRequest {
    /// Restrict to index names starting with this prefix.
    pub prefix: Option<String>,
    /// Pagination token from the previous response. Empty on the first page.
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    /// Max results per page, 0–500 (server default 100).
    #[serde(rename = "maxResults")]
    pub max_results: Option<u32>,
}

impl ListVectorIndexesRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn next_token(mut self, token: impl Into<String>) -> Self {
        self.next_token = Some(token.into());
        self
    }

    pub fn max_results(mut self, max_results: u32) -> Self {
        self.max_results = Some(max_results);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListVectorIndexesResult {
    #[serde(default)]
    pub indexes: Vec<VectorIndexInfo>,
    /// Pagination token for the next page. Absent/empty when no more pages.
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

pub struct ListVectorIndexes {
    pub request: ListVectorIndexesRequest,
}

impl Ops for ListVectorIndexes {
    type Response = BodyResponseProcessor<ListVectorIndexesResult>;
    type Body = JSONBody<ListVectorIndexesRequest>;
    type Query = ListVectorIndexesParams;

    fn prepare(self) -> Result<Prepared<ListVectorIndexesParams, ListVectorIndexesRequest>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(ListVectorIndexesParams::default()),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub trait ListVectorIndexesOps {
    /// List all vector indexes in the current vector bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listvectorindexes>
    fn list_vector_indexes(
        &self,
        request: Option<ListVectorIndexesRequest>,
    ) -> impl Future<Output = Result<ListVectorIndexesResult>>;
}

impl ListVectorIndexesOps for Client {
    async fn list_vector_indexes(
        &self,
        request: Option<ListVectorIndexesRequest>,
    ) -> Result<ListVectorIndexesResult> {
        self.request(ListVectorIndexes {
            request: request.unwrap_or_default(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&ListVectorIndexesParams::default()).unwrap(),
            "listVectorIndexes"
        );
    }

    #[test]
    fn body_serialize_empty() {
        assert_eq!(serde_json::to_string(&ListVectorIndexesRequest::new()).unwrap(), "{}");
    }

    #[test]
    fn body_serialize_full() {
        let req = ListVectorIndexesRequest::new()
            .prefix("my")
            .next_token("myindex1")
            .max_results(100);
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("\"prefix\":\"my\""));
        assert!(s.contains("\"nextToken\":\"myindex1\""));
        assert!(s.contains("\"maxResults\":100"));
    }

    #[test]
    fn parse_response() {
        let json = r#"{
          "indexes": [
            {
              "createTime": "2025-04-17T10:56:21.000Z",
              "indexName": "vectorindex1",
              "dataType": "float32",
              "dimension": 1024,
              "distanceMetric": "euclidean",
              "metadata": { "nonFilterableMetadataKeys": ["category"] },
              "status": "enable"
            }
          ],
          "nextToken": "myindex1"
        }"#;
        let parsed: ListVectorIndexesResult = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.indexes.len(), 1);
        assert_eq!(parsed.indexes[0].index_name.as_deref(), Some("vectorindex1"));
        assert_eq!(parsed.next_token.as_deref(), Some("myindex1"));
    }
}
