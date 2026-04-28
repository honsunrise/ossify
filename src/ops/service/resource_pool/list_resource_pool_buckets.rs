//! ListResourcePoolBuckets: list buckets that belong to the given resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepoolbuckets>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::ResourcePoolBucket;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListResourcePoolBucketsParams {
    #[serde(rename = "resourcePoolBuckets")]
    pub(crate) resource_pool_buckets: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,

    pub continuation_token: Option<String>,
    pub max_keys: Option<u32>,
}

impl ListResourcePoolBucketsParams {
    pub fn new(resource_pool: impl Into<String>) -> Self {
        Self {
            resource_pool_buckets: OnlyKeyField,
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
pub struct ListResourcePoolBucketsResult {
    pub resource_pool: Option<String>,
    pub continuation_token: Option<String>,
    pub next_continuation_token: Option<String>,
    #[serde(default)]
    pub is_truncated: bool,
    #[serde(default, rename = "ResourcePoolBucket")]
    pub resource_pool_buckets: Vec<ResourcePoolBucket>,
}

pub struct ListResourcePoolBuckets {
    pub params: ListResourcePoolBucketsParams,
}

impl Ops for ListResourcePoolBuckets {
    type Response = BodyResponseProcessor<ListResourcePoolBucketsResult>;
    type Body = NoneBody;
    type Query = ListResourcePoolBucketsParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<ListResourcePoolBucketsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListResourcePoolBucketsOps {
    /// List buckets inside the given resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepoolbuckets>
    fn list_resource_pool_buckets(
        &self,
        params: ListResourcePoolBucketsParams,
    ) -> impl Future<Output = Result<ListResourcePoolBucketsResult>>;
}

impl ListResourcePoolBucketsOps for Client {
    async fn list_resource_pool_buckets(
        &self,
        params: ListResourcePoolBucketsParams,
    ) -> Result<ListResourcePoolBucketsResult> {
        self.request(ListResourcePoolBuckets { params }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_defaults() {
        let q = crate::ser::to_string(&ListResourcePoolBucketsParams::new("rp-for-ai")).unwrap();
        assert_eq!(q, "resourcePool=rp-for-ai&resourcePoolBuckets");
    }

    #[test]
    fn params_serialize_with_pagination() {
        let q = crate::ser::to_string(
            &ListResourcePoolBucketsParams::new("rp-for-ai")
                .continuation_token("abc")
                .max_keys(10),
        )
        .unwrap();
        assert_eq!(
            q,
            "continuation-token=abc&max-keys=10&resourcePool=rp-for-ai&resourcePoolBuckets"
        );
    }

    #[test]
    fn parse_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListResourcePoolBucketsResult>
  <ResourcePool>rp-for-ai</ResourcePool>
  <IsTruncated>false</IsTruncated>
  <ResourcePoolBucket>
    <Name>rp-bucket-01</Name>
    <Group>test-group-1</Group>
    <JoinTime>2024-11-29T08:42:32.000Z</JoinTime>
  </ResourcePoolBucket>
</ListResourcePoolBucketsResult>"#;
        let parsed: ListResourcePoolBucketsResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.resource_pool.as_deref(), Some("rp-for-ai"));
        assert_eq!(parsed.resource_pool_buckets.len(), 1);
        assert_eq!(parsed.resource_pool_buckets[0].group.as_deref(), Some("test-group-1"));
    }
}
