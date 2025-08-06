use serde::{Deserialize, Serialize};

/// Object type
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ObjectType {
    #[default]
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Multipart")]
    Multipart,
    #[serde(rename = "Appendable")]
    Appendable,
}

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
