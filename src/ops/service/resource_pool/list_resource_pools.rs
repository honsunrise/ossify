//! ListResourcePools: list all resource pools owned by the current account.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepools>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::ResourcePool;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListResourcePoolsParams {
    #[serde(rename = "resourcePool")]
    resource_pool: OnlyKeyField,

    pub continuation_token: Option<String>,
    pub max_keys: Option<u32>,
}

impl ListResourcePoolsParams {
    pub fn new() -> Self {
        Self::default()
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
pub struct ListResourcePoolsResult {
    pub region: Option<String>,
    pub owner: Option<String>,
    pub continuation_token: Option<String>,
    pub next_continuation_token: Option<String>,
    #[serde(default)]
    pub is_truncated: bool,
    #[serde(default, rename = "ResourcePool")]
    pub resource_pools: Vec<ResourcePool>,
}

pub struct ListResourcePools {
    pub params: ListResourcePoolsParams,
}

impl Ops for ListResourcePools {
    type Response = BodyResponseProcessor<ListResourcePoolsResult>;
    type Body = NoneBody;
    type Query = ListResourcePoolsParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<ListResourcePoolsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListResourcePoolsOps {
    /// List all resource pools owned by the current account.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepools>
    fn list_resource_pools(
        &self,
        params: Option<ListResourcePoolsParams>,
    ) -> impl Future<Output = Result<ListResourcePoolsResult>>;
}

impl ListResourcePoolsOps for Client {
    async fn list_resource_pools(
        &self,
        params: Option<ListResourcePoolsParams>,
    ) -> Result<ListResourcePoolsResult> {
        self.request(ListResourcePools {
            params: params.unwrap_or_default(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_defaults() {
        assert_eq!(crate::ser::to_string(&ListResourcePoolsParams::new()).unwrap(), "resourcePool");
    }

    #[test]
    fn params_serialize_with_pagination() {
        let q = crate::ser::to_string(
            &ListResourcePoolsParams::new()
                .continuation_token("abc")
                .max_keys(5),
        )
        .unwrap();
        assert_eq!(q, "continuation-token=abc&max-keys=5&resourcePool");
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<ListResourcePools as Ops>::USE_BUCKET);
    }

    #[test]
    fn parse_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListResourcePoolsResult>
  <Region>oss-cn-hangzhou</Region>
  <Owner>103xxxx</Owner>
  <IsTruncated>false</IsTruncated>
  <ResourcePool>
    <Name>rp-for-ai</Name>
    <CreateTime>2024-11-29T08:42:32.000Z</CreateTime>
  </ResourcePool>
  <ResourcePool>
    <Name>rp-for-etl</Name>
    <CreateTime>2024-12-01T10:00:00.000Z</CreateTime>
  </ResourcePool>
</ListResourcePoolsResult>"#;
        let parsed: ListResourcePoolsResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.region.as_deref(), Some("oss-cn-hangzhou"));
        assert_eq!(parsed.resource_pools.len(), 2);
        assert_eq!(parsed.resource_pools[0].name, "rp-for-ai");
    }
}
