use std::future::Future;

use http::Method;

use crate::body::EmptyBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Request};

/// Delete bucket operation
pub struct DeleteBucket {}

impl Ops for DeleteBucket {
    type Response = EmptyResponseProcessor;
    type Body = EmptyBody;
    type Query = ();

    fn method(&self) -> Method {
        Method::DELETE
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
