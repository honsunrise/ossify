//! DeleteVectorBucket: delete a vector bucket.
//!
//! All indexes inside the bucket must be removed first, otherwise the
//! server responds with `VectorBucketNotEmpty` (409). After deletion the
//! same bucket name cannot be recreated for 4–8 hours.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletevectorbucket>

use std::future::Future;

use http::Method;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

pub struct DeleteVectorBucket;

impl Ops for DeleteVectorBucket {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = ();

    fn prepare(self) -> Result<Prepared<()>> {
        Ok(Prepared {
            method: Method::DELETE,
            ..Default::default()
        })
    }
}

pub trait DeleteVectorBucketOps {
    /// Delete the current vector bucket. Returns `VectorBucketNotEmpty`
    /// until all indexes are removed.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletevectorbucket>
    fn delete_vector_bucket(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteVectorBucketOps for Client {
    async fn delete_vector_bucket(&self) -> Result<()> {
        self.request(DeleteVectorBucket).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_method() {
        assert_eq!(DeleteVectorBucket.prepare().unwrap().method, Method::DELETE);
    }
}
