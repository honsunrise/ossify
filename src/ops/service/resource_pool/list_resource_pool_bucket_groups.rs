//! ListResourcePoolBucketGroups: list bucket-groups inside a resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepoolbucketgroups>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::ResourcePoolBucketGroup;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListResourcePoolBucketGroupsParams {
    #[serde(rename = "resourcePoolBucketGroup")]
    pub(crate) resource_pool_bucket_group: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,

    pub continuation_token: Option<String>,
    pub max_keys: Option<u32>,
}

impl ListResourcePoolBucketGroupsParams {
    pub fn new(resource_pool: impl Into<String>) -> Self {
        Self {
            resource_pool_bucket_group: OnlyKeyField,
            resource_pool: resource_pool.into(),
            continuation_token: None,
            max_keys: None,
        }
    }

    pub fn continuation_token(mut self, token: impl Into<String>) -> Self {
        self.continuation_token = Some(token.into());
        self
    }

    pub fn max_keys(mut self, max_keys: u32) -> Self {
        self.max_keys = Some(max_keys);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListResourcePoolBucketGroupsResult {
    pub resource_pool: Option<String>,
    pub continuation_token: Option<String>,
    pub next_continuation_token: Option<String>,
    #[serde(default)]
    pub is_truncated: bool,
    #[serde(default, rename = "ResourcePoolBucketGroup")]
    pub groups: Vec<ResourcePoolBucketGroup>,
}

pub struct ListResourcePoolBucketGroups {
    pub params: ListResourcePoolBucketGroupsParams,
}

impl Ops for ListResourcePoolBucketGroups {
    type Response = BodyResponseProcessor<ListResourcePoolBucketGroupsResult>;
    type Body = NoneBody;
    type Query = ListResourcePoolBucketGroupsParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<ListResourcePoolBucketGroupsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListResourcePoolBucketGroupsOps {
    /// List bucket-groups inside a resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepoolbucketgroups>
    fn list_resource_pool_bucket_groups(
        &self,
        params: ListResourcePoolBucketGroupsParams,
    ) -> impl Future<Output = Result<ListResourcePoolBucketGroupsResult>>;
}

impl ListResourcePoolBucketGroupsOps for Client {
    async fn list_resource_pool_bucket_groups(
        &self,
        params: ListResourcePoolBucketGroupsParams,
    ) -> Result<ListResourcePoolBucketGroupsResult> {
        self.request(ListResourcePoolBucketGroups { params }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&ListResourcePoolBucketGroupsParams::new("rp-for-ai")).unwrap();
        assert_eq!(q, "resourcePool=rp-for-ai&resourcePoolBucketGroup");
    }

    #[test]
    fn parse_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListResourcePoolBucketGroupsResult>
  <ResourcePool>rp-for-ai</ResourcePool>
  <IsTruncated>false</IsTruncated>
  <ResourcePoolBucketGroup>
    <Name>test-group</Name>
    <GroupBucketInfo>
      <BucketName>bucket-01</BucketName>
    </GroupBucketInfo>
    <GroupBucketInfo>
      <BucketName>bucket-02</BucketName>
    </GroupBucketInfo>
  </ResourcePoolBucketGroup>
</ListResourcePoolBucketGroupsResult>"#;
        let parsed: ListResourcePoolBucketGroupsResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.groups.len(), 1);
        assert_eq!(parsed.groups[0].name, "test-group");
        assert_eq!(parsed.groups[0].buckets.len(), 2);
        assert_eq!(parsed.groups[0].buckets[0].bucket_name, "bucket-01");
    }
}
