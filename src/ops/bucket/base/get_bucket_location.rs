use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Get bucket location request parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketLocationParams {
    location: OnlyKeyField,
}

/// Get bucket location operation
pub struct GetBucketLocation {}

impl Ops for GetBucketLocation {
    type Response = BodyResponseProcessor<String>;
    type Body = NoneBody;
    type Query = GetBucketLocationParams;

    fn prepare(self) -> Result<Prepared<GetBucketLocationParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketLocationParams {
                location: OnlyKeyField,
            }),
            ..Default::default()
        })
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
