//! DeleteBucketLogging.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketlogging>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketLoggingParams {
    logging: OnlyKeyField,
}

pub struct DeleteBucketLogging;

impl Ops for DeleteBucketLogging {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketLoggingParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketLoggingParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketLoggingParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketLoggingOps {
    /// Disable bucket access logging.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketlogging>
    fn delete_bucket_logging(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketLoggingOps for Client {
    async fn delete_bucket_logging(&self) -> Result<()> {
        self.request(DeleteBucketLogging).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&DeleteBucketLoggingParams::default()).unwrap(), "logging");
    }

    #[test]
    fn prepared_uses_delete() {
        let prepared = DeleteBucketLogging.prepare().unwrap();
        assert_eq!(prepared.method, Method::DELETE);
    }
}
