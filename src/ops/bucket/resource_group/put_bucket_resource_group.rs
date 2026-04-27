//! PutBucketResourceGroup.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketresourcegroup>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketResourceGroupParams {
    #[serde(rename = "resourceGroup")]
    resource_group: OnlyKeyField,
}

/// Root `<BucketResourceGroupConfiguration>` element.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BucketResourceGroupConfiguration", rename_all = "PascalCase")]
pub struct BucketResourceGroupConfiguration {
    pub resource_group_id: String,
}

pub struct PutBucketResourceGroup {
    pub resource_group_id: String,
}

impl Ops for PutBucketResourceGroup {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<BucketResourceGroupConfiguration>;
    type Query = PutBucketResourceGroupParams;

    fn prepare(self) -> Result<Prepared<PutBucketResourceGroupParams, BucketResourceGroupConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketResourceGroupParams::default()),
            body: Some(BucketResourceGroupConfiguration {
                resource_group_id: self.resource_group_id,
            }),
            ..Default::default()
        })
    }
}

pub trait PutBucketResourceGroupOps {
    /// Move the bucket to a different resource group. Pass the empty string
    /// to move to the default resource group.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketresourcegroup>
    fn put_bucket_resource_group(
        &self,
        resource_group_id: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutBucketResourceGroupOps for Client {
    async fn put_bucket_resource_group(&self, resource_group_id: impl Into<String>) -> Result<()> {
        self.request(PutBucketResourceGroup {
            resource_group_id: resource_group_id.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutBucketResourceGroupParams::default()).unwrap(),
            "resourceGroup"
        );
    }

    #[test]
    fn body_round_trip() {
        let cfg = BucketResourceGroupConfiguration {
            resource_group_id: "rg-abc".to_string(),
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<ResourceGroupId>rg-abc</ResourceGroupId>"));
        let back: BucketResourceGroupConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}
