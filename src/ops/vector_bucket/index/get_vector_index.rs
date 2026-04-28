//! GetVectorIndex: fetch metadata of a vector index.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getvectorindex>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::JSONBody;
use crate::error::Result;
use crate::ops::common::VectorIndexInfo;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetVectorIndexParams {
    #[serde(rename = "getVectorIndex")]
    get_vector_index: OnlyKeyField,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetVectorIndexRequest {
    #[serde(rename = "indexName")]
    pub index_name: String,
}

/// Root object of the `GetVectorIndex` response.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetVectorIndexResult {
    pub index: VectorIndexInfo,
}

pub struct GetVectorIndex {
    pub index_name: String,
}

impl Ops for GetVectorIndex {
    type Response = BodyResponseProcessor<GetVectorIndexResult>;
    type Body = JSONBody<GetVectorIndexRequest>;
    type Query = GetVectorIndexParams;

    fn prepare(self) -> Result<Prepared<GetVectorIndexParams, GetVectorIndexRequest>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(GetVectorIndexParams::default()),
            body: Some(GetVectorIndexRequest {
                index_name: self.index_name,
            }),
            ..Default::default()
        })
    }
}

pub trait GetVectorIndexOps {
    /// Fetch metadata of a vector index.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getvectorindex>
    fn get_vector_index(
        &self,
        index_name: impl Into<String>,
    ) -> impl Future<Output = Result<GetVectorIndexResult>>;
}

impl GetVectorIndexOps for Client {
    async fn get_vector_index(&self, index_name: impl Into<String>) -> Result<GetVectorIndexResult> {
        self.request(GetVectorIndex {
            index_name: index_name.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::common::{VectorDistanceMetric, VectorIndexStatus};

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetVectorIndexParams::default()).unwrap(),
            "getVectorIndex"
        );
    }

    #[test]
    fn body_serialize() {
        let req = GetVectorIndexRequest {
            index_name: "idx1".into(),
        };
        assert_eq!(serde_json::to_string(&req).unwrap(), r#"{"indexName":"idx1"}"#);
    }

    #[test]
    fn parse_response() {
        let json = r#"{
          "index": {
            "createTime": "2025-04-17T10:56:21.000Z",
            "dataType": "float32",
            "dimension": 1024,
            "distanceMetric": "euclidean",
            "metadata": { "nonFilterableMetadataKeys": ["category", "timestamp"] },
            "status": "enable"
          }
        }"#;
        let parsed: GetVectorIndexResult = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.index.dimension, Some(1024));
        assert_eq!(parsed.index.distance_metric, Some(VectorDistanceMetric::Euclidean));
        assert_eq!(parsed.index.status, Some(VectorIndexStatus::Enable));
    }
}
