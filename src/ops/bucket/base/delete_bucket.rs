use std::future::Future;

use http::Method;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// Delete bucket operation
pub struct DeleteBucket {}

impl Ops for DeleteBucket {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = ();

    fn prepare(self) -> Result<Prepared> {
        Ok(Prepared {
            method: Method::DELETE,
            ..Default::default()
        })
    }
}

pub trait DeleteBucketOps {
    /// Delete a bucket
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucket>
    fn delete_bucket(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketOps for Client {
    async fn delete_bucket(&self) -> Result<()> {
        let ops = DeleteBucket {};
        self.request(ops).await
    }
}
