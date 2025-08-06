use http::Method;
use serde::{Deserialize, Deserializer, Serialize};

use super::Owner;
use crate::body::EmptyBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListBucketsOptions {
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
    pub options: Option<ListBucketsOptions>,
}

impl Ops for ListBuckets {
    type Response = BodyResponseProcessor<ListAllMyBucketsResult>;
    type Body = EmptyBody;
    type Query = ListBucketsOptions;

    const PRODUCT: &'static str = "oss";
    const USE_BUCKET: bool = false;

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Option<&Self::Query> {
        self.options.as_ref()
    }
}

pub trait ServiceOperations {
    fn list_buckets(
        &self,
        options: Option<ListBucketsOptions>,
    ) -> impl Future<Output = Result<ListAllMyBucketsResult>>;
}

impl ServiceOperations for Client {
    async fn list_buckets(&self, options: Option<ListBucketsOptions>) -> Result<ListAllMyBucketsResult> {
        let ops = ListBuckets { options };
        self.request(ops).await
    }
}
