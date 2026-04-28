//! DeleteVectorIndex: delete a vector index and all of its vectors.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletevectorindex>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::JSONBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteVectorIndexParams {
    #[serde(rename = "deleteVectorIndex")]
    delete_vector_index: OnlyKeyField,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteVectorIndexRequest {
    #[serde(rename = "indexName")]
    pub index_name: String,
}

pub struct DeleteVectorIndex {
    pub index_name: String,
}

impl Ops for DeleteVectorIndex {
    type Response = EmptyResponseProcessor;
    type Body = JSONBody<DeleteVectorIndexRequest>;
    type Query = DeleteVectorIndexParams;

    fn prepare(self) -> Result<Prepared<DeleteVectorIndexParams, DeleteVectorIndexRequest>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(DeleteVectorIndexParams::default()),
            body: Some(DeleteVectorIndexRequest {
                index_name: self.index_name,
            }),
            ..Default::default()
        })
    }
}

pub trait DeleteVectorIndexOps {
    /// Delete a vector index. This also deletes every vector stored in the
    /// index and is irreversible.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletevectorindex>
    fn delete_vector_index(&self, index_name: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl DeleteVectorIndexOps for Client {
    async fn delete_vector_index(&self, index_name: impl Into<String>) -> Result<()> {
        self.request(DeleteVectorIndex {
            index_name: index_name.into(),
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
            crate::ser::to_string(&DeleteVectorIndexParams::default()).unwrap(),
            "deleteVectorIndex"
        );
    }

    #[test]
    fn body_serialize() {
        let req = DeleteVectorIndexRequest {
            index_name: "idx1".into(),
        };
        assert_eq!(serde_json::to_string(&req).unwrap(), r#"{"indexName":"idx1"}"#);
    }
}
