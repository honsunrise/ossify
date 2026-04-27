//! DeleteBucketCors.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketcors>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketCorsParams {
    cors: OnlyKeyField,
}

pub struct DeleteBucketCors;

impl Ops for DeleteBucketCors {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketCorsParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketCorsParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketCorsParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketCorsOps {
    /// Disable CORS and delete all CORS rules for the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketcors>
    fn delete_bucket_cors(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketCorsOps for Client {
    async fn delete_bucket_cors(&self) -> Result<()> {
        self.request(DeleteBucketCors).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&DeleteBucketCorsParams::default()).unwrap(), "cors");
    }
}
