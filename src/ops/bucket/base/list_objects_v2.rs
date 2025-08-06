use std::future::Future;

use http::Method;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;

use crate::body::EmptyBody;
use crate::error::Result;
use crate::ops::Owner;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, QueryAuthOptions, Request};

/// Object summary data for list objects v2
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectSummary {
    /// Object key name
    pub key: String,
    /// Last modified time, format like `2020-05-18T05:45:54.000Z`
    pub last_modified: String,
    /// Object ETag
    #[serde(rename = "ETag")]
    pub etag: String,
    /// Object type, usually "Normal"
    #[serde(rename = "Type")]
    pub object_type: String,
    /// File size in bytes
    pub size: u64,
    /// Storage class, such as "Standard", "IA", "Archive", "ColdArchive", etc.
    pub storage_class: String,
    /// Returned only when `fetch_owner` is set to `true` in the query
    pub owner: Option<Owner>,
    /// Object restore status, only meaningful for Archive and Cold Archive objects
    pub restore_info: Option<String>,
}

// Helper function to unwrap common prefixes from XML structure
fn unwrap_common_prefixes<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    // Handle both single prefix and array of prefixes
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct CommonPrefixes {
        #[serde(default)]
        prefix: Vec<String>,
    }

    let common_prefixes = Vec::<CommonPrefixes>::deserialize(deserializer)?;
    Ok(common_prefixes.into_iter().flat_map(|v| v.prefix).collect())
}

/// Response for list objects operation
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectsResult {
    /// Bucket name
    pub name: String,
    /// Prefix for this query result
    pub prefix: String,
    /// Maximum number of results returned in the response
    pub max_keys: u32,
    /// Character used to group object names
    pub delimiter: Option<String>,
    /// Include StartAfter element in response if StartAfter parameter was specified in the request
    pub start_after: Option<String>,
    /// Encode the returned content and specify the encoding type
    pub encoding_type: Option<String>,
    /// Whether the results returned in the request are truncated
    pub is_truncated: bool,
    /// Number of keys returned in this request
    pub key_count: u64,
    /// Include ContinuationToken element in response if ContinuationToken parameter was specified in the request
    pub continuation_token: Option<String>,
    /// Indicates that this ListObjectsV2 (GetBucketV2) request contains subsequent results
    pub next_continuation_token: Option<String>,
    /// Include CommonPrefixes element in response if Delimiter parameter was specified in the request
    #[serde(default, deserialize_with = "unwrap_common_prefixes")]
    pub common_prefixes: Vec<String>,
    /// Returned file metadata
    #[serde(default)]
    pub contents: Vec<ObjectSummary>,
}

/// Options for listing objects in a bucket
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListObjectsOptions {
    /// Character used to group object names. All object names containing the specified prefix,
    /// objects between the first occurrence of the delimiter character are treated as a group of elements
    pub delimiter: Option<String>,
    /// Set to start returning objects alphabetically after start_after.
    /// start_after is used to implement pagination, parameter length must be less than 1024 bytes
    pub start_after: Option<String>,
    /// Specify that the List operation should start from this token.
    /// You can get this token from NextContinuationToken in ListObjectsV2 (GetBucketV2) results
    pub continuation_token: Option<String>,
    /// Specify the maximum number of objects to return. Value: greater than 0 and less than or equal to 1000, default: 100
    pub max_keys: Option<u32>,
    /// Limit returned file keys to those with prefix as a prefix.
    /// Parameter length must be less than 1024 bytes
    pub prefix: Option<String>,
    /// Encode the returned content and specify the encoding type. Optional value: url
    pub encoding_type: Option<String>,
    /// Specify whether to include owner information in the results. Default: false
    pub fetch_owner: Option<bool>,
}

/// Query parameters for ListObjectsV2, includes the required list-type=2 parameter
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListObjectsV2Query {
    /// must be 2, means using ListObjectsV2 interface
    pub list_type: u8,
    #[serde(flatten)]
    pub options: Option<ListObjectsOptions>,
}

/// List objects operation
pub struct ListObjects {
    pub query: ListObjectsV2Query,
}

impl ListObjects {
    pub fn new(options: Option<ListObjectsOptions>) -> Self {
        Self {
            query: ListObjectsV2Query {
                list_type: 2, // must be 2
                options,
            },
        }
    }
}

impl Ops for ListObjects {
    type Response = BodyResponseProcessor<ListObjectsResult>;
    type Body = EmptyBody;
    type Query = ListObjectsV2Query;

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.query)
    }
}

pub trait ListObjectsOps {
    /// List objects in a bucket (V2)
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listobjectsv2>
    fn list_objects(
        &self,
        options: Option<ListObjectsOptions>,
    ) -> impl Future<Output = Result<ListObjectsResult>>;

    /// Presign list objects operation
    fn presign_list_objects(
        &self,
        public: bool,
        options: Option<ListObjectsOptions>,
        query_auth_options: QueryAuthOptions,
    ) -> impl Future<Output = Result<String>>;
}

impl ListObjectsOps for Client {
    async fn list_objects(&self, options: Option<ListObjectsOptions>) -> Result<ListObjectsResult> {
        let ops = ListObjects::new(options);
        self.request(ops).await
    }

    async fn presign_list_objects(
        &self,
        public: bool,
        options: Option<ListObjectsOptions>,
        query_auth_options: QueryAuthOptions,
    ) -> Result<String> {
        let ops = ListObjects::new(options);
        self.presign(ops, public, Some(query_auth_options)).await
    }
}
