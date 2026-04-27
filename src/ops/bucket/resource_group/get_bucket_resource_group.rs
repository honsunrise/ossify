//! GetBucketResourceGroup.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketresourcegroup>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_resource_group::BucketResourceGroupConfiguration;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketResourceGroupParams {
    #[serde(rename = "resourceGroup")]
    resource_group: OnlyKeyField,
}

pub struct GetBucketResourceGroup;

impl Ops for GetBucketResourceGroup {
    type Response = BodyResponseProcessor<BucketResourceGroupConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketResourceGroupParams;

    fn prepare(self) -> Result<Prepared<GetBucketResourceGroupParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketResourceGroupParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketResourceGroupOps {
    /// Query the resource group of the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketresourcegroup>
    fn get_bucket_resource_group(&self) -> impl Future<Output = Result<BucketResourceGroupConfiguration>>;
}

impl GetBucketResourceGroupOps for Client {
    async fn get_bucket_resource_group(&self) -> Result<BucketResourceGroupConfiguration> {
        self.request(GetBucketResourceGroup).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketResourceGroupParams::default()).unwrap(),
            "resourceGroup"
        );
    }
}
