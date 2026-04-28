//! QueryVectors: approximate-nearest-neighbor search over a vector index.
//!
//! The request supports a MongoDB-style filter DSL (operators `$eq`, `$ne`,
//! `$in`, `$nin`, `$exists`, `$and`, `$or`) against filterable metadata
//! keys. `topK` is required and limited to 1–100.
//!
//! Filters can be supplied either as a typed [`VectorFilter`] tree or as
//! raw [`serde_json::Value`] — both round-trip to the same JSON shape.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/queryvectors>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use serde_with::skip_serializing_none;

use crate::body::JSONBody;
use crate::error::Result;
use crate::ops::common::{Vector, VectorData};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryVectorsParams {
    #[serde(rename = "queryVectors")]
    query_vectors: OnlyKeyField,
}

/// Typed filter builder for `QueryVectors`.
///
/// Supports the seven operators documented by the service:
/// - `$eq` / `$ne`: single-string comparisons against a metadata field.
/// - `$in` / `$nin`: membership against a list of strings.
/// - `$exists`: field presence boolean.
/// - `$and` / `$or`: boolean composition of sub-filters (≥ 2 children each).
///
/// Builders hand back a plain `serde_json::Value` so users can freely mix
/// them with hand-written JSON.
#[derive(Debug, Clone)]
pub enum VectorFilter {
    /// `{ field: { "$eq": value } }`
    Eq { field: String, value: String },
    /// `{ field: { "$ne": value } }`
    Ne { field: String, value: String },
    /// `{ field: { "$in": [...] } }`
    In { field: String, values: Vec<String> },
    /// `{ field: { "$nin": [...] } }`
    Nin { field: String, values: Vec<String> },
    /// `{ field: { "$exists": true|false } }`
    Exists { field: String, exists: bool },
    /// `{ "$and": [...] }`
    And(Vec<VectorFilter>),
    /// `{ "$or": [...] }`
    Or(Vec<VectorFilter>),
    /// Escape hatch for arbitrary filter shapes.
    Raw(Value),
}

impl VectorFilter {
    pub fn eq(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Eq {
            field: field.into(),
            value: value.into(),
        }
    }

    pub fn ne(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Ne {
            field: field.into(),
            value: value.into(),
        }
    }

    pub fn is_in(field: impl Into<String>, values: Vec<String>) -> Self {
        Self::In {
            field: field.into(),
            values,
        }
    }

    pub fn not_in(field: impl Into<String>, values: Vec<String>) -> Self {
        Self::Nin {
            field: field.into(),
            values,
        }
    }

    pub fn exists(field: impl Into<String>, exists: bool) -> Self {
        Self::Exists {
            field: field.into(),
            exists,
        }
    }

    pub fn and(filters: Vec<VectorFilter>) -> Self {
        Self::And(filters)
    }

    pub fn or(filters: Vec<VectorFilter>) -> Self {
        Self::Or(filters)
    }

    /// Convert this filter tree into its JSON representation.
    pub fn into_json(self) -> Value {
        match self {
            VectorFilter::Eq { field, value } => json!({ field: { "$eq": value } }),
            VectorFilter::Ne { field, value } => json!({ field: { "$ne": value } }),
            VectorFilter::In { field, values } => json!({ field: { "$in": values } }),
            VectorFilter::Nin { field, values } => json!({ field: { "$nin": values } }),
            VectorFilter::Exists { field, exists } => json!({ field: { "$exists": exists } }),
            VectorFilter::And(filters) => json!({
                "$and": filters.into_iter().map(|f| f.into_json()).collect::<Vec<_>>()
            }),
            VectorFilter::Or(filters) => json!({
                "$or": filters.into_iter().map(|f| f.into_json()).collect::<Vec<_>>()
            }),
            VectorFilter::Raw(v) => v,
        }
    }
}

/// JSON body of [`QueryVectors`].
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct QueryVectorsRequest {
    #[serde(rename = "indexName")]
    pub index_name: String,
    #[serde(rename = "queryVector")]
    pub query_vector: VectorData,
    /// Top-K neighbors to return. Required; valid range 1–100.
    #[serde(rename = "topK")]
    pub top_k: u32,
    /// Optional metadata filter expression (MongoDB-style).
    pub filter: Option<Value>,
    /// Include distances in the response. Defaults to false.
    #[serde(rename = "returnDistance")]
    pub return_distance: Option<bool>,
    /// Include metadata in the response. Defaults to false.
    #[serde(rename = "returnMetadata")]
    pub return_metadata: Option<bool>,
}

impl QueryVectorsRequest {
    pub fn new(index_name: impl Into<String>, query_vector: VectorData, top_k: u32) -> Self {
        Self {
            index_name: index_name.into(),
            query_vector,
            top_k,
            filter: None,
            return_distance: None,
            return_metadata: None,
        }
    }

    pub fn filter(mut self, filter: VectorFilter) -> Self {
        self.filter = Some(filter.into_json());
        self
    }

    /// Attach a raw JSON filter. Use this when you need an operator or shape
    /// that isn't covered by [`VectorFilter`].
    pub fn raw_filter(mut self, value: Value) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn return_distance(mut self, value: bool) -> Self {
        self.return_distance = Some(value);
        self
    }

    pub fn return_metadata(mut self, value: bool) -> Self {
        self.return_metadata = Some(value);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QueryVectorsResult {
    /// Neighbors in order of decreasing similarity (smaller `distance` first).
    #[serde(default)]
    pub vectors: Vec<Vector>,
}

pub struct QueryVectors {
    pub request: QueryVectorsRequest,
}

impl Ops for QueryVectors {
    type Response = BodyResponseProcessor<QueryVectorsResult>;
    type Body = JSONBody<QueryVectorsRequest>;
    type Query = QueryVectorsParams;

    fn prepare(self) -> Result<Prepared<QueryVectorsParams, QueryVectorsRequest>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(QueryVectorsParams::default()),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub trait QueryVectorsOps {
    /// Find the top-K nearest neighbors of `query_vector` in `index_name`,
    /// optionally filtered by metadata.
    ///
    /// Lower `distance` values indicate higher similarity. Freshly written
    /// vectors typically become visible to queries after 2–3 seconds.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/queryvectors>
    fn query_vectors(&self, request: QueryVectorsRequest)
    -> impl Future<Output = Result<QueryVectorsResult>>;
}

impl QueryVectorsOps for Client {
    async fn query_vectors(&self, request: QueryVectorsRequest) -> Result<QueryVectorsResult> {
        self.request(QueryVectors { request }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&QueryVectorsParams::default()).unwrap(), "queryVectors");
    }

    #[test]
    fn filter_builder_eq() {
        let v = VectorFilter::eq("year", "2020").into_json();
        assert_eq!(v, json!({ "year": { "$eq": "2020" } }));
    }

    #[test]
    fn filter_builder_in_and_exists() {
        let v = VectorFilter::and(vec![
            VectorFilter::is_in("category", vec!["tech".into(), "sci".into()]),
            VectorFilter::exists("reviewed", true),
        ])
        .into_json();
        assert_eq!(
            v,
            json!({
                "$and": [
                    { "category": { "$in": ["tech", "sci"] } },
                    { "reviewed": { "$exists": true } }
                ]
            })
        );
    }

    #[test]
    fn body_serialize_with_filter() {
        let req = QueryVectorsRequest::new("idx1", VectorData::new(vec![0.1, 0.2, 0.3]), 5)
            .filter(VectorFilter::eq("year", "2020"))
            .return_distance(true)
            .return_metadata(true);
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("\"indexName\":\"idx1\""));
        assert!(s.contains("\"topK\":5"));
        assert!(s.contains("\"queryVector\":{\"float32\":[0.1,0.2,0.3]}"));
        assert!(s.contains("\"filter\":{\"year\":{\"$eq\":\"2020\"}}"));
        assert!(s.contains("\"returnDistance\":true"));
        assert!(s.contains("\"returnMetadata\":true"));
    }

    #[test]
    fn body_skips_optional_fields() {
        let req = QueryVectorsRequest::new("idx1", VectorData::new(vec![0.1]), 1);
        let s = serde_json::to_string(&req).unwrap();
        assert!(!s.contains("filter"));
        assert!(!s.contains("returnDistance"));
        assert!(!s.contains("returnMetadata"));
    }

    #[test]
    fn parse_response() {
        let json = r#"{
          "vectors": [
            {
              "key": "doc-001",
              "distance": 0.12,
              "metadata": { "title": "hello", "year": "2020" }
            },
            {
              "key": "doc-003",
              "distance": 0.25
            }
          ]
        }"#;
        let parsed: QueryVectorsResult = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.vectors.len(), 2);
        assert_eq!(parsed.vectors[0].key, "doc-001");
        assert_eq!(parsed.vectors[0].distance, Some(0.12));
        assert!(parsed.vectors[0].metadata.is_some());
        assert_eq!(parsed.vectors[1].distance, Some(0.25));
        assert!(parsed.vectors[1].metadata.is_none());
    }
}
