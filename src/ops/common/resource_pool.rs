//! Shared XML types for account-level Resource Pool QoS APIs.

use serde::{Deserialize, Serialize};

/// One entry in `<ListResourcePoolsResult>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResourcePool {
    pub name: String,
    pub create_time: Option<String>,
}

/// One entry in `<ListResourcePoolBucketsResult>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResourcePoolBucket {
    pub name: String,
    /// The bucket-group this bucket belongs to, if any.
    pub group: Option<String>,
    pub join_time: Option<String>,
}

/// One `<GroupBucketInfo>` entry inside `<ResourcePoolBucketGroup>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GroupBucketInfo {
    pub bucket_name: String,
}

/// One entry in `<ListResourcePoolBucketGroupsResult>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResourcePoolBucketGroup {
    pub name: String,
    #[serde(default, rename = "GroupBucketInfo")]
    pub buckets: Vec<GroupBucketInfo>,
}
