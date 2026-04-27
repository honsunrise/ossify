//! GetBucketCors.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketcors>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_cors::CorsConfiguration;
#[allow(unused_imports)]
pub use super::put_bucket_cors::CorsRule;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketCorsParams {
    cors: OnlyKeyField,
}

pub struct GetBucketCors;

impl Ops for GetBucketCors {
    type Response = BodyResponseProcessor<CorsConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketCorsParams;

    fn prepare(self) -> Result<Prepared<GetBucketCorsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketCorsParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketCorsOps {
    /// Query the bucket CORS configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketcors>
    fn get_bucket_cors(&self) -> impl Future<Output = Result<CorsConfiguration>>;
}

impl GetBucketCorsOps for Client {
    async fn get_bucket_cors(&self) -> Result<CorsConfiguration> {
        self.request(GetBucketCors).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetBucketCorsParams::default()).unwrap(), "cors");
    }
}
