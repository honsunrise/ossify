//! GetBucketLifecycle.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketlifecycle>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::LifecycleConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketLifecycleParams {
    lifecycle: OnlyKeyField,
}

pub struct GetBucketLifecycle;

impl Ops for GetBucketLifecycle {
    type Response = BodyResponseProcessor<LifecycleConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketLifecycleParams;

    fn prepare(self) -> Result<Prepared<GetBucketLifecycleParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketLifecycleParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketLifecycleOps {
    /// Query the lifecycle rules of a bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketlifecycle>
    fn get_bucket_lifecycle(&self) -> impl Future<Output = Result<LifecycleConfiguration>>;
}

impl GetBucketLifecycleOps for Client {
    async fn get_bucket_lifecycle(&self) -> Result<LifecycleConfiguration> {
        self.request(GetBucketLifecycle).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_adds_lifecycle_subresource() {
        let q = crate::ser::to_string(&GetBucketLifecycleParams::default()).unwrap();
        assert_eq!(q, "lifecycle");
    }

    #[test]
    fn prepared_uses_get() {
        let prepared = GetBucketLifecycle.prepare().unwrap();
        assert_eq!(prepared.method, Method::GET);
        assert!(prepared.body.is_none());
    }
}
