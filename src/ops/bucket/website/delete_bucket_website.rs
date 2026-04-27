//! DeleteBucketWebsite.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketwebsite>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketWebsiteParams {
    website: OnlyKeyField,
}

pub struct DeleteBucketWebsite;

impl Ops for DeleteBucketWebsite {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketWebsiteParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketWebsiteParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketWebsiteParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketWebsiteOps {
    /// Disable static website hosting and clear routing rules.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketwebsite>
    fn delete_bucket_website(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketWebsiteOps for Client {
    async fn delete_bucket_website(&self) -> Result<()> {
        self.request(DeleteBucketWebsite).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&DeleteBucketWebsiteParams::default()).unwrap(), "website");
    }
}
