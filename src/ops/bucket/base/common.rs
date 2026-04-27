use serde::{Deserialize, Serialize};

// Re-export the canonical ObjectType from `ops::common` for backwards
// compatibility within the bucket module.
pub use crate::ops::common::ObjectType;

/// Summary information of a bucket
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketSummary {
    pub name: String,
    pub location: String,
    pub creation_date: String,
    pub extranet_endpoint: String,
    pub intranet_endpoint: String,
    pub region: String,
    pub storage_class: String,
    pub resource_group_id: Option<String>,
    pub comment: Option<String>,
}

/// Options for listing buckets
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListBucketsOptions {
    pub prefix: Option<String>,
    pub marker: Option<String>,
    pub max_keys: Option<u32>,
    pub resource_group_id: Option<String>,
}
