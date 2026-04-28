//! GetVectors: batch read vectors by key.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getvectors>

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
pub struct GetVectorsParams {
    #[serde(rename = "getVectors")]
    get_vectors: OnlyKeyField,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetVectorsRequest {
    #[serde(rename = "indexName")]
    pub index_name: String,
    /// 1–100 keys; duplicates allowed; missing keys are silently skipped.
    pub keys: Vec<String>,
    /// Include vector data in the response. Defaults to false.
    #[serde(rename = "returnData")]
    pub return_data: Option<bool>,
    /// Include metadata in the response. Defaults to false.
    #[serde(rename = "returnMetadata")]
    pub return_metadata: Option<bool>,
}

impl GetVectorsRequest {
    pub fn new(index_name: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            index_name: index_name.into(),
            keys,
            return_data: None,
            return_metadata: None,
        }
    }

    pub fn return_data(mut self, value: bool) -> Self {
        self.return_data = Some(value);
        self
    }

    pub fn return_metadata(mut self, value: bool) -> Self {
        self.return_metadata = Some(value);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetVectorsResult {
    #[serde(default)]
    pub vectors: Vec<Vector>,
}

pub struct GetVectors {
    pub request: GetVectorsRequest,
}

impl Ops for GetVectors {
    type Response = BodyResponseProcessor<GetVectorsResult>;
    type Body = JSONBody<GetVectorsRequest>;
    type Query = GetVectorsParams;

    fn prepare(self) -> Result<Prepared<GetVectorsParams, GetVectorsRequest>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(GetVectorsParams::default()),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub trait GetVectorsOps {
    /// Batch-read vectors by key (1–100 keys per call).
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getvectors>
    fn get_vectors(&self, request: GetVectorsRequest) -> impl Future<Output = Result<GetVectorsResult>>;
}

impl GetVectorsOps for Client {
    async fn get_vectors(&self, request: GetVectorsRequest) -> Result<GetVectorsResult> {
        self.request(GetVectors { request }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetVectorsParams::default()).unwrap(), "getVectors");
    }

    #[test]
    fn body_serialize() {
        let req = GetVectorsRequest::new("idx1", vec!["k1".into(), "k2".into()])
            .return_data(true)
            .return_metadata(true);
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("\"indexName\":\"idx1\""));
        assert!(s.contains("\"keys\":[\"k1\",\"k2\"]"));
        assert!(s.contains("\"returnData\":true"));
        assert!(s.contains("\"returnMetadata\":true"));
    }

    #[test]
    fn parse_response() {
        let json = r#"{
          "vectors": [
            {
              "key": "doc-001",
              "data": { "float32": [0.1, 0.2, 0.3, 0.4, 0.5] },
              "metadata": { "title": "hello" }
            }
          ]
        }"#;
        let parsed: GetVectorsResult = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.vectors.len(), 1);
        assert_eq!(parsed.vectors[0].key, "doc-001");
        assert_eq!(parsed.vectors[0].data.as_ref().unwrap().float32.len(), 5);
    }
}
