use std::future::Future;

use http::Method;

use crate::body::EmptyBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

/// Get bucket location operation
pub struct GetBucketLocation {}

impl Ops for GetBucketLocation {
    type Response = BodyResponseProcessor<String>;
    type Body = EmptyBody;
    type Query = ();

    fn method(&self) -> Method {
        Method::GET
    }
}

pub trait GetBucketLocationOps {
    /// Get bucket location
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketlocation>
    fn get_bucket_location(&self) -> impl Future<Output = Result<String>>;
}

impl GetBucketLocationOps for Client {
    async fn get_bucket_location(&self) -> Result<String> {
        let ops = GetBucketLocation {};
        self.request(ops).await
    }
}
