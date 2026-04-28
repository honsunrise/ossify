//! PutVectorIndex: create a vector index inside the current vector bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putvectorindex>

use std::future::Future;

use http::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::body::JSONBody;
use crate::error::Result;
use crate::ops::common::{VectorDataType, VectorDistanceMetric, VectorIndexMetadata};
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`PutVectorIndex`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutVectorIndexParams {
    #[serde(rename = "putVectorIndex")]
    put_vector_index: OnlyKeyField,
}

/// JSON body of [`PutVectorIndex`].
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct PutVectorIndexRequest {
    #[serde(rename = "indexName")]
    pub index_name: String,
    /// 1–4096; typical default 512.
    pub dimension: u32,
    #[serde(rename = "dataType")]
    pub data_type: VectorDataType,
    #[serde(rename = "distanceMetric")]
    pub distance_metric: VectorDistanceMetric,
    pub metadata: Option<VectorIndexMetadata>,
}

impl PutVectorIndexRequest {
    pub fn new(index_name: impl Into<String>, dimension: u32, distance_metric: VectorDistanceMetric) -> Self {
        Self {
            index_name: index_name.into(),
            dimension,
            data_type: VectorDataType::Float32,
            distance_metric,
            metadata: None,
        }
    }

    /// Declare the metadata keys that must NOT be used in query filters
    /// (at most 10).
    pub fn non_filterable_metadata_keys(mut self, keys: Vec<String>) -> Self {
        self.metadata = Some(VectorIndexMetadata {
            non_filterable_metadata_keys: Some(keys),
        });
        self
    }
}

pub struct PutVectorIndex {
    pub request: PutVectorIndexRequest,
}

impl Ops for PutVectorIndex {
    type Response = EmptyResponseProcessor;
    type Body = JSONBody<PutVectorIndexRequest>;
    type Query = PutVectorIndexParams;

    fn prepare(self) -> Result<Prepared<PutVectorIndexParams, PutVectorIndexRequest>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(PutVectorIndexParams::default()),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub trait PutVectorIndexOps {
    /// Create a vector index.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putvectorindex>
    fn put_vector_index(&self, request: PutVectorIndexRequest) -> impl Future<Output = Result<()>>;
}

impl PutVectorIndexOps for Client {
    async fn put_vector_index(&self, request: PutVectorIndexRequest) -> Result<()> {
        self.request(PutVectorIndex { request }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutVectorIndexParams::default()).unwrap(),
            "putVectorIndex"
        );
    }

    #[test]
    fn body_round_trip() {
        let req = PutVectorIndexRequest::new("idx1", 1024, VectorDistanceMetric::Cosine)
            .non_filterable_metadata_keys(vec!["category".into(), "timestamp".into()]);
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("\"indexName\":\"idx1\""));
        assert!(s.contains("\"dataType\":\"float32\""));
        assert!(s.contains("\"distanceMetric\":\"cosine\""));
        assert!(s.contains("\"nonFilterableMetadataKeys\":[\"category\",\"timestamp\"]"));
    }

    #[test]
    fn body_skips_none_metadata() {
        let req = PutVectorIndexRequest::new("idx1", 512, VectorDistanceMetric::Euclidean);
        let s = serde_json::to_string(&req).unwrap();
        assert!(!s.contains("metadata"));
    }
}
