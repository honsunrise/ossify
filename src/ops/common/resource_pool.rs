//! Shared XML types for account-level Resource Pool QoS APIs.

use serde::{Deserialize, Serialize};

use super::QoSConfiguration;

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

/// One `<ResourcePoolBucketGroupQoSInfo>` entry, returned by Get/List APIs.
///
/// Note: The official documentation inconsistently names the inner "group
/// name" element. Both `<BucketGroup>` (as used in response examples) and
/// `<ResourcePoolBucketGroup>` are accepted here.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "ResourcePoolBucketGroupQoSInfo", rename_all = "PascalCase")]
pub struct ResourcePoolBucketGroupQoSInfo {
    /// Bucket-group name. Populated from either `<BucketGroup>` or
    /// `<ResourcePoolBucketGroup>` on the wire.
    #[serde(rename = "BucketGroup", alias = "ResourcePoolBucketGroup")]
    pub bucket_group: String,
    #[serde(rename = "QoSConfiguration")]
    pub qos_configuration: QoSConfiguration,
}
