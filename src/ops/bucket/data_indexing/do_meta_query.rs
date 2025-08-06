use std::collections::HashMap;
use std::future::Future;

use http::Method;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Request};

/// Query mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DoMetaQueryMode {
    /// Basic query
    Basic,
    /// Semantic query (vector search)
    Semantic,
}

impl Default for DoMetaQueryMode {
    fn default() -> Self {
        Self::Basic
    }
}

impl AsRef<str> for DoMetaQueryMode {
    fn as_ref(&self) -> &str {
        match self {
            DoMetaQueryMode::Basic => "basic",
            DoMetaQueryMode::Semantic => "semantic",
        }
    }
}

/// Sort order
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Asc
    }
}

impl AsRef<str> for SortOrder {
    fn as_ref(&self) -> &str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}

/// Media type (used for vector search)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
}

impl AsRef<str> for MediaType {
    fn as_ref(&self) -> &str {
        match self {
            MediaType::Image => "image",
            MediaType::Video => "video",
            MediaType::Audio => "audio",
            MediaType::Document => "document",
        }
    }
}

/// Aggregation operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AggregationOperation {
    /// Calculate sum
    Sum,
    /// Calculate average
    Avg,
    /// Calculate maximum
    Max,
    /// Calculate minimum
    Min,
    /// Calculate count
    Count,
    /// Group statistics
    Group,
}

impl AsRef<str> for AggregationOperation {
    fn as_ref(&self) -> &str {
        match self {
            AggregationOperation::Sum => "sum",
            AggregationOperation::Avg => "avg",
            AggregationOperation::Max => "max",
            AggregationOperation::Min => "min",
            AggregationOperation::Count => "count",
            AggregationOperation::Group => "group",
        }
    }
}

/// Aggregation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Aggregation {
    /// Aggregation field
    pub field: String,
    /// Aggregation operation
    pub operation: AggregationOperation,
}

/// Aggregation result
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AggregationResult {
    /// Aggregation field
    pub field: String,
    /// Aggregation operation
    pub operation: String,
    /// Aggregation value
    pub value: Option<String>,
    /// Group results (only when operation is group)
    #[serde(default)]
    pub groups: Vec<AggregationGroup>,
}

/// Aggregated group results
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AggregationGroup {
    /// Group value
    pub value: String,
    /// Group count
    pub count: u64,
}

/// Address information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    pub address_line: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub district: Option<String>,
    pub language: Option<String>,
    pub province: Option<String>,
    pub township: Option<String>,
}

/// Video stream information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VideoStream {
    pub codec_name: Option<String>,
    pub language: Option<String>,
    pub bitrate: Option<String>,
    pub frame_rate: Option<String>,
    pub start_time: Option<String>,
    pub duration: Option<String>,
    pub frame_count: Option<String>,
    pub bit_depth: Option<String>,
    pub pixel_format: Option<String>,
    pub color_space: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

/// Audio stream information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AudioStream {
    pub codec_name: Option<String>,
    pub bitrate: Option<String>,
    pub sample_rate: Option<String>,
    pub start_time: Option<String>,
    pub duration: Option<String>,
    pub channels: Option<String>,
    pub language: Option<String>,
}

/// Subtitle information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Subtitle {
    pub codec_name: Option<String>,
    pub language: Option<String>,
    pub start_time: Option<String>,
    pub duration: Option<String>,
}

/// File information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileInfo {
    /// File name
    pub filename: String,
    /// File size (bytes)
    pub size: u64,
    /// File last modified time
    pub file_modified_time: Option<String>,
    /// Object last modified time
    #[serde(rename = "OSSObjectLastModifiedTime")]
    pub oss_object_last_modified_time: Option<String>,
    /// ETag
    #[serde(rename = "ETag")]
    pub etag: Option<String>,
    /// CRC64
    #[serde(rename = "OSSCRC64")]
    pub oss_crc64: Option<String>,
    /// Creation time
    pub produce_time: Option<String>,
    /// Content type
    pub content_type: Option<String>,
    /// Media type
    pub media_type: Option<String>,
    /// Longitude and latitude coordinates
    pub lat_long: Option<String>,
    /// Title
    pub title: Option<String>,
    /// Expiration time
    #[serde(rename = "OSSExpiration")]
    pub oss_expiration: Option<String>,
    /// Cache control
    pub cache_control: Option<String>,
    /// Content description
    pub content_disposition: Option<String>,
    /// Content encoding
    pub content_encoding: Option<String>,
    /// Content language
    pub content_language: Option<String>,
    /// Access control allow origin
    pub access_control_allow_origin: Option<String>,
    /// Access control request method
    pub access_control_request_method: Option<String>,
    /// Server-side data encryption
    pub server_side_data_encryption: Option<String>,
    /// Server-side encryption key ID
    pub server_side_encryption_key_id: Option<String>,
    /// Image height
    pub image_height: Option<u32>,
    /// Image width
    pub image_width: Option<u32>,
    /// Video width
    pub video_width: Option<u32>,
    /// Video height
    pub video_height: Option<u32>,
    /// Bit rate
    pub bitrate: Option<String>,
    /// Artist
    pub artist: Option<String>,
    /// Album artist
    pub album_artist: Option<String>,
    /// Composer
    pub composer: Option<String>,
    /// Performer
    pub performer: Option<String>,
    /// Album
    pub album: Option<String>,
    /// Duration
    pub duration: Option<String>,
    /// Object type
    #[serde(rename = "OSSObjectType")]
    pub oss_object_type: Option<String>,
    /// Storage class
    #[serde(rename = "OSSStorageClass")]
    pub oss_storage_class: Option<String>,
    /// Tag count
    #[serde(rename = "OSSTaggingCount")]
    pub oss_tagging_count: Option<u32>,
    /// Video stream information
    #[serde(default)]
    pub video_streams: Vec<VideoStream>,
    /// Audio stream information
    #[serde(default)]
    pub audio_streams: Vec<AudioStream>,
    /// Subtitle information
    #[serde(default)]
    pub subtitles: Vec<Subtitle>,
    /// Address information
    #[serde(default)]
    pub addresses: Vec<Address>,
    /// Object tags
    #[serde(rename = "OSSTagging", default, deserialize_with = "unwrap_tagging")]
    pub tagging: HashMap<String, String>,
    /// User metadata
    #[serde(rename = "OSSUserMeta", default, deserialize_with = "unwrap_user_meta")]
    pub user_meta: HashMap<String, String>,
}

/// MetaQuery body
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename = "MetaQuery", rename_all = "PascalCase")]
pub struct MetaQueryBody {
    /// Pagination token
    pub next_token: Option<String>,
    /// Maximum number of objects to return
    pub max_results: Option<u32>,
    /// Query condition (Basic query)
    pub query: Option<String>,
    /// Query condition (Vector search)
    pub simple_query: Option<String>,
    /// Sort field
    pub sort: Option<String>,
    /// Sort order
    pub order: Option<SortOrder>,
    /// Aggregation operation
    pub aggregations: Option<Vec<Aggregation>>,
    /// Media type (used for Vector search)
    pub media_types: Option<Vec<MediaType>>,
}

fn unwrap_files<'de, D>(deserializer: D) -> std::result::Result<Vec<FileInfo>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Files {
        #[serde(default)]
        file: Vec<FileInfo>,
    }
    Ok(Files::deserialize(deserializer)?.file)
}

fn unwrap_aggregations<'de, D>(deserializer: D) -> std::result::Result<Vec<AggregationResult>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Aggregations {
        #[serde(default)]
        aggregation: Vec<AggregationResult>,
    }
    Ok(Aggregations::deserialize(deserializer)?.aggregation)
}

fn unwrap_user_meta<'de, D>(de: D) -> std::result::Result<HashMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    /// User metadata
    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct UserMeta {
        pub key: String,
        pub value: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct OSSUserMeta {
        #[serde(default)]
        user_meta: Vec<UserMeta>,
    }
    Ok(OSSUserMeta::deserialize(de)?
        .user_meta
        .into_iter()
        .map(|meta| {
            (
                meta.key
                    .to_lowercase()
                    .trim_start_matches("x-oss-meta-")
                    .to_string(),
                meta.value,
            )
        })
        .collect())
}

fn unwrap_tagging<'de, D>(de: D) -> std::result::Result<HashMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    /// Object tags
    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Tagging {
        pub key: String,
        pub value: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct OSSTagging {
        #[serde(default)]
        tagging: Vec<Tagging>,
    }
    Ok(OSSTagging::deserialize(de)?
        .tagging
        .into_iter()
        .map(|t| (t.key, t.value))
        .collect())
}

fn empty_string_as_none<'de, D, T>(de: D) -> std::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

/// MetaQuery response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetaQueryResponse {
    /// Next page token
    #[serde(deserialize_with = "empty_string_as_none")]
    // #[serde_as(as = "NoneAsEmptyString")]
    pub next_token: Option<String>,
    /// File list
    #[serde(default, deserialize_with = "unwrap_files")]
    pub files: Vec<FileInfo>,
    /// Aggregation result
    #[serde(default, deserialize_with = "unwrap_aggregations")]
    pub aggregations: Vec<AggregationResult>,
}

/// DoMetaQuery request parameters
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DoMetaQueryParams {
    pub mode: DoMetaQueryMode,
    pub comp: String,
    meta_query: OnlyKeyField,
}

impl Default for DoMetaQueryParams {
    fn default() -> Self {
        Self {
            meta_query: OnlyKeyField,
            mode: DoMetaQueryMode::Basic,
            comp: "query".to_string(),
        }
    }
}

/// DoMetaQuery operation
pub struct DoMetaQuery {
    pub mode: DoMetaQueryMode,
    pub body: MetaQueryBody,
    pub query: DoMetaQueryParams,
}

impl Ops for DoMetaQuery {
    type Response = BodyResponseProcessor<MetaQueryResponse>;
    type Body = XMLBody<MetaQueryBody>;
    type Query = DoMetaQueryParams;

    const PRODUCT: &'static str = "oss";

    fn method(&self) -> Method {
        Method::POST
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.query)
    }

    fn body(&self) -> Option<&MetaQueryBody> {
        Some(&self.body)
    }
}

/// DoMetaQueryOperations trait
pub trait DataIndexingOperations {
    /// Query files (objects) that meet the specified conditions
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/dometaquery>
    fn do_meta_query(
        &self,
        mode: DoMetaQueryMode,
        body: MetaQueryBody,
        query: DoMetaQueryParams,
    ) -> impl Future<Output = Result<MetaQueryResponse>>;
}

impl DataIndexingOperations for Client {
    async fn do_meta_query(
        &self,
        mode: DoMetaQueryMode,
        body: MetaQueryBody,
        query: DoMetaQueryParams,
    ) -> Result<MetaQueryResponse> {
        let ops = DoMetaQuery { mode, body, query };
        self.request(ops).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// Query builder for constructing complex query conditions
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    conditions: Vec<QueryCondition>,
}

/// Query condition
#[derive(Debug, Clone)]
pub struct QueryCondition {
    pub field: String,
    pub operation: String,
    pub value: String,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    pub fn eq(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.push(QueryCondition {
            field: field.into(),
            operation: "eq".to_string(),
            value: value.into(),
        });
        self
    }

    pub fn gt(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.push(QueryCondition {
            field: field.into(),
            operation: "gt".to_string(),
            value: value.into(),
        });
        self
    }

    pub fn gte(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.push(QueryCondition {
            field: field.into(),
            operation: "gte".to_string(),
            value: value.into(),
        });
        self
    }

    pub fn lt(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.push(QueryCondition {
            field: field.into(),
            operation: "lt".to_string(),
            value: value.into(),
        });
        self
    }

    pub fn lte(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.push(QueryCondition {
            field: field.into(),
            operation: "lte".to_string(),
            value: value.into(),
        });
        self
    }

    pub fn prefix(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.push(QueryCondition {
            field: field.into(),
            operation: "prefix".to_string(),
            value: value.into(),
        });
        self
    }

    pub fn r#match(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.push(QueryCondition {
            field: field.into(),
            operation: "match".to_string(),
            value: value.into(),
        });
        self
    }

    pub fn build(self) -> Result<String> {
        if self.conditions.is_empty() {
            return Err(crate::error::Error::InvalidArgument(
                "Query conditions cannot be empty".to_string(),
            ));
        }

        if self.conditions.len() == 1 {
            let condition = &self.conditions[0];
            let query = serde_json::json!({
                "Field": condition.field,
                "Operation": condition.operation,
                "Value": condition.value
            });
            Ok(query.to_string())
        } else {
            let sub_queries: Vec<_> = self
                .conditions
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "Field": c.field,
                        "Operation": c.operation,
                        "Value": c.value
                    })
                })
                .collect();

            let query = serde_json::json!({
                "Operation": "and",
                "SubQueries": sub_queries
            });
            Ok(query.to_string())
        }
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// MetaQueryRequest builder
#[derive(Debug, Clone)]
pub struct MetaQueryRequestBuilder {
    request: MetaQueryBody,
}

impl MetaQueryRequestBuilder {
    /// Create a new request builder
    pub fn new() -> Self {
        Self {
            request: MetaQueryBody {
                next_token: None,
                max_results: None,
                query: None,
                simple_query: None,
                sort: None,
                order: None,
                aggregations: None,
                media_types: None,
            },
        }
    }

    /// Set pagination token
    pub fn next_token(mut self, token: impl Into<String>) -> Self {
        self.request.next_token = Some(token.into());
        self
    }

    /// Set maximum number of results
    pub fn max_results(mut self, max: u32) -> Self {
        self.request.max_results = Some(max);
        self
    }

    /// Set query condition (for Basic query)
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.request.query = Some(query.into());
        self
    }

    /// Set simple query condition (for Vector search)
    pub fn simple_query(mut self, query: impl Into<String>) -> Self {
        self.request.simple_query = Some(query.into());
        self
    }

    /// Set sort field
    pub fn sort(mut self, field: impl Into<String>) -> Self {
        self.request.sort = Some(field.into());
        self
    }

    /// Set sort order
    pub fn order(mut self, order: SortOrder) -> Self {
        self.request.order = Some(order);
        self
    }

    /// Add aggregation operation
    pub fn aggregation(mut self, field: impl Into<String>, operation: AggregationOperation) -> Self {
        let aggregations = self.request.aggregations.get_or_insert_with(Vec::new);
        aggregations.push(Aggregation {
            field: field.into(),
            operation,
        });
        self
    }

    /// Set media type (for Vector search)
    pub fn media_types(mut self, types: Vec<MediaType>) -> Self {
        self.request.media_types = Some(types);
        self
    }

    /// Build request
    pub fn build(self) -> MetaQueryBody {
        self.request
    }
}

impl Default for MetaQueryRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
