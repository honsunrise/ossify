//! DeleteBucketPublicAccessBlock.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketpublicaccessblock>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketPublicAccessBlockParams {
    #[serde(rename = "publicAccessBlock")]
    public_access_block: OnlyKeyField,
}

pub struct DeleteBucketPublicAccessBlock;

impl Ops for DeleteBucketPublicAccessBlock {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketPublicAccessBlockParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketPublicAccessBlockParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketPublicAccessBlockParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketPublicAccessBlockOps {
    /// Delete the bucket-level Block Public Access configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketpublicaccessblock>
    fn delete_bucket_public_access_block(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketPublicAccessBlockOps for Client {
    async fn delete_bucket_public_access_block(&self) -> Result<()> {
        self.request(DeleteBucketPublicAccessBlock).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&DeleteBucketPublicAccessBlockParams::default()).unwrap(),
            "publicAccessBlock"
        );
    }

    #[test]
    fn method_is_delete() {
        let p = DeleteBucketPublicAccessBlock.prepare().unwrap();
        assert_eq!(p.method, Method::DELETE);
    }
}
