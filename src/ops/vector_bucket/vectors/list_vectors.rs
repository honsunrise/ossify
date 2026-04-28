//! ListVectors: paginate (optionally in parallel) through all vectors in an
//! index.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listvectors>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::JSONBody;
use crate::error::Result;
use crate::ops::common::Vector;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListVectorsParams {
    #[serde(rename = "listVectors")]
    list_vectors: OnlyKeyField,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct ListVectorsRequest {
    #[serde(rename = "indexName")]
    pub index_name: String,
    /// Page size, default 500, max 1000.
    #[serde(rename = "maxResults")]
    pub max_results: Option<u32>,
    /// Pagination token (1–2048 bytes).
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    /// Include vector data in the response. Defaults to false.
    #[serde(rename = "returnData")]
    pub return_data: Option<bool>,
    /// Include metadata in the response. Defaults to false.
    #[serde(rename = "returnMetadata")]
    pub return_metadata: Option<bool>,
    /// Parallel-scan total segment count (max 16). Set to 1 for serial scan.
    #[serde(rename = "segmentCount")]
    pub segment_count: Option<u32>,
    /// Current segment index; must be `< segmentCount`.
    #[serde(rename = "segmentIndex")]
    pub segment_index: Option<u32>,
}

impl ListVectorsRequest {
    pub fn new(index_name: impl Into<String>) -> Self {
        Self {
            index_name: index_name.into(),
            max_results: None,
            next_token: None,
            return_data: None,
            return_metadata: None,
            segment_count: None,
            segment_index: None,
        }
    }

    pub fn max_results(mut self, value: u32) -> Self {
        self.max_results = Some(value);
        self
    }

    pub fn next_token(mut self, value: impl Into<String>) -> Self {
        self.next_token = Some(value.into());
        self
    }

    pub fn return_data(mut self, value: bool) -> Self {
        self.return_data = Some(value);
        self
    }

    pub fn return_metadata(mut self, value: bool) -> Self {
        self.return_metadata = Some(value);
        self
    }

    /// Configure parallel scan by providing the total number of segments and
    /// the index (0-based) of the current segment.
    pub fn segment(mut self, segment_count: u32, segment_index: u32) -> Self {
        self.segment_count = Some(segment_count);
        self.segment_index = Some(segment_index);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListVectorsResult {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(default)]
    pub vectors: Vec<Vector>,
}

pub struct ListVectors {
    pub request: ListVectorsRequest,
}

impl Ops for ListVectors {
    type Response = BodyResponseProcessor<ListVectorsResult>;
    type Body = JSONBody<ListVectorsRequest>;
    type Query = ListVectorsParams;

    fn prepare(self) -> Result<Prepared<ListVectorsParams, ListVectorsRequest>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(ListVectorsParams::default()),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub trait ListVectorsOps {
    /// Paginate (optionally in parallel) through every vector in an index.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listvectors>
    fn list_vectors(&self, request: ListVectorsRequest) -> impl Future<Output = Result<ListVectorsResult>>;
}

impl ListVectorsOps for Client {
    async fn list_vectors(&self, request: ListVectorsRequest) -> Result<ListVectorsResult> {
        self.request(ListVectors { request }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&ListVectorsParams::default()).unwrap(), "listVectors");
    }

    #[test]
    fn body_serialize_minimal() {
        let req = ListVectorsRequest::new("idx1");
        let s = serde_json::to_string(&req).unwrap();
        assert_eq!(s, r#"{"indexName":"idx1"}"#);
    }

    #[test]
    fn body_serialize_with_segment() {
        let req = ListVectorsRequest::new("idx1")
            .max_results(500)
            .return_data(true)
            .return_metadata(false)
            .segment(4, 2);
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("\"segmentCount\":4"));
        assert!(s.contains("\"segmentIndex\":2"));
        assert!(s.contains("\"returnData\":true"));
        assert!(s.contains("\"returnMetadata\":false"));
    }

    #[test]
    fn parse_response() {
        let json = r#"{
          "nextToken": "tok-2",
          "vectors": [
            { "key": "a", "data": { "float32": [0.1, 0.2] } }
          ]
        }"#;
        let parsed: ListVectorsResult = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.next_token.as_deref(), Some("tok-2"));
        assert_eq!(parsed.vectors.len(), 1);
    }
}
