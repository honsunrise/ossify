use http::Method;
use serde::{Deserialize, Deserializer, Serialize};

use super::Owner;
use crate::body::NoneBody;
use crate::error::Result;
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

pub trait ServiceOperations {
    /// Lists all buckets that belong to your Alibaba Cloud account.
    /// You can specify the prefix, marker, or max-keys parameter to list buckets that meet specific conditions.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbuckets>
    fn list_buckets(
        &self,
        params: Option<ListBucketsParams>,
    ) -> impl Future<Output = Result<ListAllMyBucketsResult>>;
}

impl ServiceOperations for Client {
    async fn list_buckets(&self, params: Option<ListBucketsParams>) -> Result<ListAllMyBucketsResult> {
        let ops = ListBuckets {
            params: params.unwrap_or_default(),
        };
        self.request(ops).await
    }
}
