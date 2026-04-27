//! DeleteBucketLifecycle.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketlifecycle>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketLifecycleParams {
    lifecycle: OnlyKeyField,
}

pub struct DeleteBucketLifecycle;

impl Ops for DeleteBucketLifecycle {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketLifecycleParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketLifecycleParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketLifecycleParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketLifecycleOps {
    /// Delete all lifecycle rules for a bucket. Objects are not affected.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketlifecycle>
    fn delete_bucket_lifecycle(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketLifecycleOps for Client {
    async fn delete_bucket_lifecycle(&self) -> Result<()> {
        self.request(DeleteBucketLifecycle).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_adds_lifecycle_subresource() {
        let q = crate::ser::to_string(&DeleteBucketLifecycleParams::default()).unwrap();
        assert_eq!(q, "lifecycle");
    }

    #[test]
    fn prepared_uses_delete() {
        let prepared = DeleteBucketLifecycle.prepare().unwrap();
        assert_eq!(prepared.method, Method::DELETE);
    }
}
