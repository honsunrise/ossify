//! DeleteVectors: batch-delete vectors by key.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletevectors>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::JSONBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteVectorsParams {
    #[serde(rename = "deleteVectors")]
    delete_vectors: OnlyKeyField,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteVectorsRequest {
    #[serde(rename = "indexName")]
    pub index_name: String,
    /// 1–500 unique keys. Keys that don't exist are silently skipped.
    pub keys: Vec<String>,
}

pub struct DeleteVectors {
    pub request: DeleteVectorsRequest,
}

impl Ops for DeleteVectors {
    type Response = EmptyResponseProcessor;
    type Body = JSONBody<DeleteVectorsRequest>;
    type Query = DeleteVectorsParams;

    fn prepare(self) -> Result<Prepared<DeleteVectorsParams, DeleteVectorsRequest>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(DeleteVectorsParams::default()),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub trait DeleteVectorsOps {
    /// Batch-delete up to 500 vectors by key. Missing keys are silently
    /// skipped. The operation is not atomic — a 5xx response can leave
    /// partial deletions.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletevectors>
    fn delete_vectors(&self, request: DeleteVectorsRequest) -> impl Future<Output = Result<()>>;
}

impl DeleteVectorsOps for Client {
    async fn delete_vectors(&self, request: DeleteVectorsRequest) -> Result<()> {
        self.request(DeleteVectors { request }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&DeleteVectorsParams::default()).unwrap(), "deleteVectors");
    }

    #[test]
    fn body_serialize() {
        let req = DeleteVectorsRequest {
            index_name: "idx1".into(),
            keys: vec!["a".into(), "b".into(), "c".into()],
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("\"indexName\":\"idx1\""));
        assert!(s.contains("\"keys\":[\"a\",\"b\",\"c\"]"));
    }
}
