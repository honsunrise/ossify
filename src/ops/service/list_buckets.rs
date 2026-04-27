use std::future::Future;

use http::Method;
use serde::{Deserialize, Deserializer, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::Owner;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListBucketsParams {
    pub marker: Option<String>,
    pub max_keys: Option<u32>,
    pub prefix: Option<String>,
    pub resource_group_id: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    pub name: String,
    pub region: String,
    pub location: String,
    pub creation_date: String,
    pub storage_class: String,
    pub extranet_endpoint: String,
    pub intranet_endpoint: String,
    pub comment: Option<String>,
    pub resource_group_id: Option<String>,
}

fn unwrap_buckets<'de, D>(deserializer: D) -> std::result::Result<Vec<Bucket>, D::Error>
where
    D: Deserializer<'de>,
{
    /// Represents <Buckets>...</Buckets>
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Buckets {
        // default allows empty list
        #[serde(default)]
        bucket: Vec<Bucket>,
    }
    Ok(Buckets::deserialize(deserializer)?.bucket)
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyBucketsResult {
    pub prefix: Option<String>,
    pub marker: Option<String>,
    pub max_keys: Option<i32>,
    pub is_truncated: Option<bool>,
    pub next_marker: Option<String>,
    pub owner: Owner,
    #[serde(deserialize_with = "unwrap_buckets")]
    pub buckets: Vec<Bucket>,
}

pub struct ListBuckets {
    pub params: ListBucketsParams,
}

impl Ops for ListBuckets {
    type Response = BodyResponseProcessor<ListAllMyBucketsResult>;
    type Body = NoneBody;
    type Query = ListBucketsParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<ListBucketsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListBucketsOps {
    /// Lists all buckets that belong to your Alibaba Cloud account.
    /// You can specify the prefix, marker, or max-keys parameter to list buckets that meet specific conditions.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbuckets>
    fn list_buckets(
        &self,
        params: Option<ListBucketsParams>,
    ) -> impl Future<Output = Result<ListAllMyBucketsResult>>;
}

impl ListBucketsOps for Client {
    async fn list_buckets(&self, params: Option<ListBucketsParams>) -> Result<ListAllMyBucketsResult> {
        let ops = ListBuckets {
            params: params.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_empty() {
        let q = crate::ser::to_string(&ListBucketsParams::default()).unwrap();
        assert_eq!(q, "");
    }

    #[test]
    fn params_serialize_with_fields() {
        let q = crate::ser::to_string(&ListBucketsParams {
            marker: Some("m".to_string()),
            max_keys: Some(10),
            prefix: Some("p".to_string()),
            resource_group_id: Some("rg-1".to_string()),
        })
        .unwrap();
        assert_eq!(q, "marker=m&max-keys=10&prefix=p&resource-group-id=rg-1");
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<ListBuckets as Ops>::USE_BUCKET);
    }

    #[test]
    fn parse_empty_list() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListAllMyBucketsResult>
  <Owner>
    <ID>1234</ID>
    <DisplayName>user</DisplayName>
  </Owner>
  <Buckets></Buckets>
</ListAllMyBucketsResult>"#;
        let parsed: ListAllMyBucketsResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.buckets.len(), 0);
        assert_eq!(parsed.owner.id.as_deref(), Some("1234"));
    }

    #[test]
    fn parse_two_buckets() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListAllMyBucketsResult>
  <Owner>
    <ID>1234</ID>
    <DisplayName>user</DisplayName>
  </Owner>
  <Buckets>
    <Bucket>
      <Name>bucket-a</Name>
      <Region>cn-hangzhou</Region>
      <Location>oss-cn-hangzhou</Location>
      <CreationDate>2023-01-01T00:00:00.000Z</CreationDate>
      <StorageClass>Standard</StorageClass>
      <ExtranetEndpoint>oss-cn-hangzhou.aliyuncs.com</ExtranetEndpoint>
      <IntranetEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</IntranetEndpoint>
    </Bucket>
    <Bucket>
      <Name>bucket-b</Name>
      <Region>cn-shanghai</Region>
      <Location>oss-cn-shanghai</Location>
      <CreationDate>2024-01-01T00:00:00.000Z</CreationDate>
      <StorageClass>IA</StorageClass>
      <ExtranetEndpoint>oss-cn-shanghai.aliyuncs.com</ExtranetEndpoint>
      <IntranetEndpoint>oss-cn-shanghai-internal.aliyuncs.com</IntranetEndpoint>
      <Comment>second bucket</Comment>
      <ResourceGroupId>rg-xyz</ResourceGroupId>
    </Bucket>
  </Buckets>
</ListAllMyBucketsResult>"#;
        let parsed: ListAllMyBucketsResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.buckets.len(), 2);
        assert_eq!(parsed.buckets[0].name, "bucket-a");
        assert_eq!(parsed.buckets[1].resource_group_id.as_deref(), Some("rg-xyz"));
        assert_eq!(parsed.buckets[1].comment.as_deref(), Some("second bucket"));
    }
}
