//! PutVectors: batch insert (or overwrite) vectors in an index.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putvectors>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::JSONBody;
use crate::error::Result;
use crate::ops::common::Vector;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutVectorsParams {
    #[serde(rename = "putVectors")]
    put_vectors: OnlyKeyField,
}

/// JSON body of [`PutVectors`]. `vectors` must contain 1–500 elements.
#[derive(Debug, Clone, Serialize)]
pub struct PutVectorsRequest {
    #[serde(rename = "indexName")]
    pub index_name: String,
    pub vectors: Vec<Vector>,
}

pub struct PutVectors {
    pub request: PutVectorsRequest,
}

impl Ops for PutVectors {
    type Response = EmptyResponseProcessor;
    type Body = JSONBody<PutVectorsRequest>;
    type Query = PutVectorsParams;

    fn prepare(self) -> Result<Prepared<PutVectorsParams, PutVectorsRequest>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(PutVectorsParams::default()),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub trait PutVectorsOps {
    /// Batch-insert or overwrite up to 500 vectors in a single call.
    ///
    /// Duplicate keys already present in the index are overwritten; duplicate
    /// keys *within the same batch* cause the whole batch to fail. The
    /// operation is not atomic — a 5xx response can leave partial writes.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putvectors>
    fn put_vectors(&self, request: PutVectorsRequest) -> impl Future<Output = Result<()>>;
}

impl PutVectorsOps for Client {
    async fn put_vectors(&self, request: PutVectorsRequest) -> Result<()> {
        self.request(PutVectors { request }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::common::VectorData;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&PutVectorsParams::default()).unwrap(), "putVectors");
    }

    #[test]
    fn body_serialize() {
        let req = PutVectorsRequest {
            index_name: "idx1".into(),
            vectors: vec![Vector {
                key: "k1".into(),
                data: Some(VectorData::new(vec![0.1, 0.2, 0.3])),
                metadata: None,
                distance: None,
            }],
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("\"indexName\":\"idx1\""));
        assert!(s.contains("\"key\":\"k1\""));
        assert!(s.contains("\"float32\":[0.1,0.2,0.3]"));
    }
}
