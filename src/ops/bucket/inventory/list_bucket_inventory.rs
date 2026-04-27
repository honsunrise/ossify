//! ListBucketInventory.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbucketinventory>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::InventoryConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListBucketInventoryParams {
    inventory: OnlyKeyField,
    pub continuation_token: Option<String>,
}

impl ListBucketInventoryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn continuation_token(mut self, token: impl Into<String>) -> Self {
        self.continuation_token = Some(token.into());
        self
    }
}

/// Response body (XML root `<ListInventoryConfigurationsResult>`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename = "ListInventoryConfigurationsResult", rename_all = "PascalCase")]
pub struct ListInventoryConfigurationsResult {
    #[serde(rename = "InventoryConfiguration", default)]
    pub inventory_configurations: Vec<InventoryConfiguration>,
    #[serde(default)]
    pub is_truncated: bool,
    pub next_continuation_token: Option<String>,
}

pub struct ListBucketInventory {
    pub params: ListBucketInventoryParams,
}

impl Ops for ListBucketInventory {
    type Response = BodyResponseProcessor<ListInventoryConfigurationsResult>;
    type Body = NoneBody;
    type Query = ListBucketInventoryParams;

    fn prepare(self) -> Result<Prepared<ListBucketInventoryParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListBucketInventoryOps {
    /// List all inventory rules on the bucket (paginated, up to 100 per page).
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbucketinventory>
    fn list_bucket_inventory(
        &self,
        params: Option<ListBucketInventoryParams>,
    ) -> impl Future<Output = Result<ListInventoryConfigurationsResult>>;
}

impl ListBucketInventoryOps for Client {
    async fn list_bucket_inventory(
        &self,
        params: Option<ListBucketInventoryParams>,
    ) -> Result<ListInventoryConfigurationsResult> {
        self.request(ListBucketInventory {
            params: params.unwrap_or_default(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_default() {
        assert_eq!(
            crate::ser::to_string(&ListBucketInventoryParams::default()).unwrap(),
            "inventory"
        );
    }

    #[test]
    fn params_serialize_with_token() {
        let q = crate::ser::to_string(&ListBucketInventoryParams::new().continuation_token("tkn")).unwrap();
        assert_eq!(q, "continuation-token=tkn&inventory");
    }

    #[test]
    fn parse_response() {
        let xml = r#"<ListInventoryConfigurationsResult>
 <InventoryConfiguration>
    <Id>report1</Id>
    <IsEnabled>true</IsEnabled>
    <Destination>
       <OSSBucketDestination>
          <Format>CSV</Format>
          <AccountId>1000000000000000</AccountId>
          <RoleArn>acs:ram::1000000000000000:role/AliyunOSSRole</RoleArn>
          <Bucket>acs:oss:::destination-bucket</Bucket>
          <Prefix>prefix1</Prefix>
       </OSSBucketDestination>
    </Destination>
    <Schedule>
       <Frequency>Daily</Frequency>
    </Schedule>
    <IncludedObjectVersions>All</IncludedObjectVersions>
 </InventoryConfiguration>
 <InventoryConfiguration>
    <Id>report2</Id>
    <IsEnabled>false</IsEnabled>
    <Destination>
       <OSSBucketDestination>
          <Format>CSV</Format>
          <AccountId>1</AccountId>
          <RoleArn>r</RoleArn>
          <Bucket>b</Bucket>
       </OSSBucketDestination>
    </Destination>
    <Schedule>
       <Frequency>Weekly</Frequency>
    </Schedule>
    <IncludedObjectVersions>Current</IncludedObjectVersions>
 </InventoryConfiguration>
 <IsTruncated>true</IsTruncated>
 <NextContinuationToken>abc</NextContinuationToken>
</ListInventoryConfigurationsResult>"#;
        let parsed: ListInventoryConfigurationsResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.inventory_configurations.len(), 2);
        assert!(parsed.is_truncated);
        assert_eq!(parsed.next_continuation_token.as_deref(), Some("abc"));
    }
}
