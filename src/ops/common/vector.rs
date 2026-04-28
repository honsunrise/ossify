//! Shared JSON types for Vector Bucket APIs.
//!
//! Vector Bucket operations live on a dedicated `oss-vectors.aliyuncs.com`
//! endpoint and exchange `application/json` payloads using camelCase field
//! names (except the bucket-info responses, which preserve PascalCase names
//! from the legacy OSS XML schema for compatibility).
//!
//! Official category index:
//! <https://www.alibabacloud.com/help/en/oss/developer-reference/apis-for-operations-on-vector-buckets/>

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Vector element data-type. Only `float32` is currently supported.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VectorDataType {
    #[default]
    Float32,
}

/// Distance metric used by a vector index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VectorDistanceMetric {
    /// Euclidean (L2) distance. Default when not specified.
    Euclidean,
    /// Cosine distance.
    Cosine,
}

impl Default for VectorDistanceMetric {
    fn default() -> Self {
        Self::Euclidean
    }
}

/// Lifecycle status of a vector index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VectorIndexStatus {
    Creating,
    Enable,
    Deleting,
    /// Forward-compatibility escape hatch for any status value the server may
    /// introduce in the future.
    #[serde(untagged)]
    Other(String),
}

/// Metadata block passed in `PutVectorIndex` and echoed back in
/// `GetVectorIndex` / `ListVectorIndexes`.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorIndexMetadata {
    /// Metadata keys that must not be used in query filters. At most 10 keys;
    /// each 1–63 bytes, alphanumerics + underscore, must start with a letter
    /// or underscore.
    #[serde(rename = "nonFilterableMetadataKeys")]
    pub non_filterable_metadata_keys: Option<Vec<String>>,
}

/// One vector index, as returned by `GetVectorIndex` (nested under `index`)
/// and `ListVectorIndexes` (each element of `indexes`).
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorIndexInfo {
    /// Index name. `GetVectorIndex` may omit this field; `ListVectorIndexes`
    /// always sets it.
    #[serde(rename = "indexName")]
    pub index_name: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "dataType")]
    pub data_type: Option<VectorDataType>,
    pub dimension: Option<u32>,
    #[serde(rename = "distanceMetric")]
    pub distance_metric: Option<VectorDistanceMetric>,
    pub metadata: Option<VectorIndexMetadata>,
    pub status: Option<VectorIndexStatus>,
}

/// Raw vector payload. Only `float32` is supported currently, but the JSON
/// schema keeps the data-type as a wrapper object so future types can be
/// added without a breaking change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorData {
    pub float32: Vec<f32>,
}

impl VectorData {
    pub fn new(values: Vec<f32>) -> Self {
        Self { float32: values }
    }
}

/// A single vector, used in write and query requests/responses.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    /// Primary key (1–1024 UTF-8 bytes). Required on writes; always present
    /// on reads.
    pub key: String,
    /// Vector contents. Always present on writes, and on reads when
    /// `returnData` is true.
    pub data: Option<VectorData>,
    /// Arbitrary per-vector metadata. Total size ≤ 40 KB; filterable portion
    /// ≤ 2 KB with at most 10 keys.
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Set only in `QueryVectors` responses when `returnDistance` is true.
    /// Lower values indicate higher similarity.
    pub distance: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_info_round_trip() {
        let json = r#"{
          "indexName": "idx1",
          "createTime": "2025-04-17T10:56:21.000Z",
          "dataType": "float32",
          "dimension": 1024,
          "distanceMetric": "cosine",
          "metadata": { "nonFilterableMetadataKeys": ["category"] },
          "status": "enable"
        }"#;
        let parsed: VectorIndexInfo = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.index_name.as_deref(), Some("idx1"));
        assert_eq!(parsed.dimension, Some(1024));
        assert_eq!(parsed.distance_metric, Some(VectorDistanceMetric::Cosine));
        assert_eq!(parsed.status, Some(VectorIndexStatus::Enable));
    }

    #[test]
    fn vector_round_trip_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("title".to_string(), serde_json::json!("hello"));
        metadata.insert("tags".to_string(), serde_json::json!(["ai", "vector"]));
        let v = Vector {
            key: "k1".to_string(),
            data: Some(VectorData::new(vec![0.1, 0.2, 0.3])),
            metadata: Some(metadata),
            distance: None,
        };
        let s = serde_json::to_string(&v).unwrap();
        let back: Vector = serde_json::from_str(&s).unwrap();
        assert_eq!(back.key, "k1");
        assert_eq!(back.data.as_ref().unwrap().float32.len(), 3);
        assert_eq!(back.metadata.unwrap().len(), 2);
    }

    #[test]
    fn unknown_status_is_preserved() {
        let v: VectorIndexStatus = serde_json::from_str("\"frozen\"").unwrap();
        match v {
            VectorIndexStatus::Other(s) => assert_eq!(s, "frozen"),
            _ => panic!("expected Other variant"),
        }
    }
}
