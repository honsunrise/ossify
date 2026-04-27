//! GetBucketWebsite.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketwebsite>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_website::WebsiteConfiguration;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketWebsiteParams {
    website: OnlyKeyField,
}

pub struct GetBucketWebsite;

impl Ops for GetBucketWebsite {
    type Response = BodyResponseProcessor<WebsiteConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketWebsiteParams;

    fn prepare(self) -> Result<Prepared<GetBucketWebsiteParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketWebsiteParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketWebsiteOps {
    /// Retrieve the static-website-hosting configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketwebsite>
    fn get_bucket_website(&self) -> impl Future<Output = Result<WebsiteConfiguration>>;
}

impl GetBucketWebsiteOps for Client {
    async fn get_bucket_website(&self) -> Result<WebsiteConfiguration> {
        self.request(GetBucketWebsite).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetBucketWebsiteParams::default()).unwrap(), "website");
    }
}
