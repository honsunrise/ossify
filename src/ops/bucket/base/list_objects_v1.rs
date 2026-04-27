//! GetBucket (ListObjects V1): list all objects in a bucket using the legacy
//! marker-based pagination.
//!
//! New applications should prefer [`ListObjects`](super::ListObjects) (V2),
//! which supports continuation tokens. V1 is kept for compatibility with
//! existing integrations that rely on `Marker` / `NextMarker`.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listobjects>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::{EncodingType, ObjectType, Owner, StorageClass};
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Prepared, QueryAuthOptions, Request};

/// Query parameters for [`ListObjectsV1`].
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListObjectsV1Params {
    /// Character used to group object names into `CommonPrefixes`.
    pub delimiter: Option<String>,
    /// Start listing from the object alphabetically after `marker`.
    pub marker: Option<String>,
    /// Maximum objects to return (1-1000, default 100).
    pub max_keys: Option<u32>,
    /// Return only objects whose key begins with this prefix.
    pub prefix: Option<String>,
    /// Encoding applied to the returned Delimiter, Marker, Prefix, NextMarker,
    /// and Key fields. Only `url` is supported.
    pub encoding_type: Option<EncodingType>,
}

impl ListObjectsV1Params {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }

    pub fn marker(mut self, marker: impl Into<String>) -> Self {
        self.marker = Some(marker.into());
        self
    }

    pub fn max_keys(mut self, max_keys: u32) -> Self {
        self.max_keys = Some(max_keys);
        self
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn encoding_type(mut self, encoding_type: EncodingType) -> Self {
        self.encoding_type = Some(encoding_type);
        self
    }
}

/// Per-object metadata entry inside a [`ListObjectsV1Result`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectSummaryV1 {
    /// Object key (name).
    pub key: String,
    /// Time the object was last modified, e.g. `2012-02-24T08:42:32.000Z`.
    pub last_modified: String,
    /// Entity tag - MD5 for simple uploads, non-MD5 for other methods.
    #[serde(rename = "ETag")]
    pub etag: String,
    /// Object type: `Normal`, `Multipart`, `Appendable`, or `Symlink`.
    #[serde(rename = "Type")]
    pub object_type: ObjectType,
    /// Size in bytes.
    pub size: u64,
    /// Storage class.
    pub storage_class: StorageClass,
    /// Set when the object has been transitioned to Cold Archive / Deep Cold
    /// Archive by a lifecycle rule.
    pub transition_time: Option<String>,
    /// Set when a sealed append object has been sealed.
    pub sealed_time: Option<String>,
    /// Restore status for Cold / Deep Cold Archive objects. Values look like
    /// `ongoing-request="true"` or
    /// `ongoing-request="false", expiry-date="Thu, 24 Sep 2020 12:40:33 GMT"`.
    pub restore_info: Option<String>,
    /// Owner information.
    pub owner: Option<Owner>,
}

fn deserialize_opt_u32<'de, D>(deserializer: D) -> std::result::Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    // OSS occasionally returns an empty <MaxKeys></MaxKeys> element (see the
    // archive-bucket example in the official docs). serde's default
    // `Option<u32>` deserialization would fail with "invalid type: string \"\"",
    // so we accept an optional string and parse it when non-empty.
    let s = Option::<String>::deserialize(deserializer)?;
    match s.as_deref() {
        None | Some("") => Ok(None),
        Some(v) => v.parse::<u32>().map(Some).map_err(serde::de::Error::custom),
    }
}

fn unwrap_common_prefixes<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct CommonPrefixes {
        #[serde(default)]
        prefix: Vec<String>,
    }

    let common_prefixes = Vec::<CommonPrefixes>::deserialize(deserializer)?;
    Ok(common_prefixes.into_iter().flat_map(|v| v.prefix).collect())
}

/// Response body for [`ListObjectsV1`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectsV1Result {
    /// Bucket name.
    pub name: String,
    /// Echo of the request `prefix`. Omitted (empty) when the request did not
    /// specify one.
    pub prefix: Option<String>,
    /// Echo of the request `marker`.
    pub marker: Option<String>,
    /// Echo of the request `max-keys`. Some OSS error responses (e.g. archive
    /// listing samples in the docs) emit an empty `<MaxKeys></MaxKeys>` element,
    /// so this is modelled as optional rather than `u32`.
    #[serde(default, deserialize_with = "deserialize_opt_u32")]
    pub max_keys: Option<u32>,
    /// Echo of the request `delimiter`.
    pub delimiter: Option<String>,
    /// Echo of the request `encoding-type`, if any.
    pub encoding_type: Option<EncodingType>,
    /// Whether the listing was truncated.
    pub is_truncated: bool,
    /// Marker to pass as `marker` in the next request when `is_truncated` is
    /// true. Note V1 uses `NextMarker`, not `NextContinuationToken`.
    pub next_marker: Option<String>,
    /// Individual objects.
    #[serde(default)]
    pub contents: Vec<ObjectSummaryV1>,
    /// Common prefixes when `delimiter` was specified.
    #[serde(default, deserialize_with = "unwrap_common_prefixes")]
    pub common_prefixes: Vec<String>,
}

/// The `ListObjects` V1 (GetBucket) operation.
pub struct ListObjectsV1 {
    pub query: ListObjectsV1Params,
}

impl ListObjectsV1 {
    pub fn new(params: Option<ListObjectsV1Params>) -> Self {
        Self {
            query: params.unwrap_or_default(),
        }
    }
}

impl Ops for ListObjectsV1 {
    type Response = BodyResponseProcessor<ListObjectsV1Result>;
    type Body = NoneBody;
    type Query = ListObjectsV1Params;

    fn prepare(self) -> Result<Prepared<ListObjectsV1Params>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.query),
            ..Default::default()
        })
    }
}

pub trait ListObjectsV1Ops {
    /// List objects in a bucket using the legacy V1 (GetBucket) protocol.
    ///
    /// New integrations should prefer [`ListObjectsOps::list_objects`](super::ListObjectsOps::list_objects)
    /// (the V2 API). V1 exists for compatibility.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listobjects>
    fn list_objects_v1(
        &self,
        params: Option<ListObjectsV1Params>,
    ) -> impl Future<Output = Result<ListObjectsV1Result>>;

    /// Presign a list-objects V1 URL.
    fn presign_list_objects_v1(
        &self,
        public: bool,
        params: Option<ListObjectsV1Params>,
        query_auth_options: QueryAuthOptions,
    ) -> impl Future<Output = Result<String>>;
}

impl ListObjectsV1Ops for Client {
    async fn list_objects_v1(&self, params: Option<ListObjectsV1Params>) -> Result<ListObjectsV1Result> {
        let ops = ListObjectsV1::new(params);
        self.request(ops).await
    }

    async fn presign_list_objects_v1(
        &self,
        public: bool,
        params: Option<ListObjectsV1Params>,
        query_auth_options: QueryAuthOptions,
    ) -> Result<String> {
        let ops = ListObjectsV1::new(params);
        self.presign(ops, public, Some(query_auth_options)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_empty() {
        let q = crate::ser::to_string(&ListObjectsV1Params::new()).unwrap();
        assert_eq!(q, "");
    }

    #[test]
    fn params_serialize_all_fields() {
        let q = crate::ser::to_string(
            &ListObjectsV1Params::new()
                .delimiter("/")
                .marker("m")
                .max_keys(50)
                .prefix("fun/")
                .encoding_type(EncodingType::Url),
        )
        .unwrap();
        // Alphabetical order per ser::MapSerializer.
        assert_eq!(q, "delimiter=%2F&encoding-type=url&marker=m&max-keys=50&prefix=fun%2F");
    }

    #[test]
    fn prepared_uses_get_without_key() {
        let prepared = ListObjectsV1::new(None).prepare().unwrap();
        assert_eq!(prepared.method, Method::GET);
        assert!(prepared.key.is_none());
    }

    #[test]
    fn parse_simple_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult>
<Name>examplebucket</Name>
<Prefix></Prefix>
<Marker></Marker>
<MaxKeys>100</MaxKeys>
<Delimiter></Delimiter>
<IsTruncated>false</IsTruncated>
<Contents>
  <Key>fun/test.jpg</Key>
  <LastModified>2012-02-24T08:42:32.000Z</LastModified>
  <ETag>"5B3C1A2E053D763E1B002CC607C5A0FE1****"</ETag>
  <Type>Normal</Type>
  <Size>344606</Size>
  <StorageClass>Standard</StorageClass>
  <Owner>
    <ID>0022012****</ID>
    <DisplayName>user-example</DisplayName>
  </Owner>
</Contents>
</ListBucketResult>"#;
        let parsed: ListObjectsV1Result = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.name, "examplebucket");
        assert_eq!(parsed.max_keys, Some(100));
        assert!(!parsed.is_truncated);
        assert_eq!(parsed.contents.len(), 1);
        assert_eq!(parsed.contents[0].key, "fun/test.jpg");
        assert_eq!(parsed.contents[0].size, 344606);
        assert_eq!(parsed.contents[0].object_type, ObjectType::Normal);
        assert_eq!(parsed.contents[0].storage_class, StorageClass::Standard);
    }

    #[test]
    fn parse_response_with_common_prefixes() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult>
<Name>examplebucket</Name>
<Prefix>fun/</Prefix>
<Marker></Marker>
<MaxKeys>100</MaxKeys>
<Delimiter>/</Delimiter>
<IsTruncated>false</IsTruncated>
<Contents>
  <Key>fun/test.jpg</Key>
  <LastModified>2012-02-24T08:42:32.000Z</LastModified>
  <ETag>"abc"</ETag>
  <Type>Normal</Type>
  <Size>100</Size>
  <StorageClass>Standard</StorageClass>
</Contents>
<CommonPrefixes>
  <Prefix>fun/movie/</Prefix>
</CommonPrefixes>
</ListBucketResult>"#;
        let parsed: ListObjectsV1Result = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.common_prefixes, vec!["fun/movie/".to_string()]);
        assert_eq!(parsed.contents.len(), 1);
    }

    #[test]
    fn parse_paginated_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult>
<Name>examplebucket</Name>
<Prefix></Prefix>
<Marker>test1.txt</Marker>
<MaxKeys>2</MaxKeys>
<Delimiter></Delimiter>
<EncodingType>url</EncodingType>
<IsTruncated>true</IsTruncated>
<NextMarker>test100.txt</NextMarker>
<Contents>
  <Key>test10.txt</Key>
  <LastModified>2020-05-26T07:50:18.000Z</LastModified>
  <ETag>"abc"</ETag>
  <Type>Normal</Type>
  <Size>1</Size>
  <StorageClass>Standard</StorageClass>
</Contents>
</ListBucketResult>"#;
        let parsed: ListObjectsV1Result = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.is_truncated);
        assert_eq!(parsed.next_marker.as_deref(), Some("test100.txt"));
        assert_eq!(parsed.encoding_type, Some(EncodingType::Url));
    }

    #[test]
    fn parse_response_with_restore_info() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult>
<Name>examplebucket</Name>
<Prefix></Prefix>
<Marker></Marker>
<MaxKeys>100</MaxKeys>
<Delimiter></Delimiter>
<IsTruncated>false</IsTruncated>
<Contents>
  <Key>ex.txt</Key>
  <LastModified>2020-06-22T11:42:32.000Z</LastModified>
  <ETag>"e"</ETag>
  <Type>Normal</Type>
  <Size>1</Size>
  <StorageClass>Standard</StorageClass>
  <RestoreInfo>ongoing-request="false", expiry-date="Thu, 24 Sep 2020 12:40:33 GMT"</RestoreInfo>
</Contents>
</ListBucketResult>"#;
        let parsed: ListObjectsV1Result = quick_xml::de::from_str(xml).unwrap();
        assert!(
            parsed.contents[0]
                .restore_info
                .as_ref()
                .unwrap()
                .contains("ongoing-request")
        );
    }

    #[test]
    fn parse_response_with_transition_and_sealed_times() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult>
<Name>examplebucket</Name>
<MaxKeys>100</MaxKeys>
<IsTruncated>false</IsTruncated>
<Contents>
  <Key>movie/001.avi</Key>
  <TransitionTime>2024-04-23T07:21:42.000Z</TransitionTime>
  <LastModified>2012-02-24T08:43:07.000Z</LastModified>
  <ETag>"e"</ETag>
  <Type>Normal</Type>
  <Size>1</Size>
  <StorageClass>ColdArchive</StorageClass>
</Contents>
<Contents>
  <Key>sealed-append.log</Key>
  <LastModified>2020-05-21T12:07:15.000Z</LastModified>
  <SealedTime>2020-05-21T12:07:15.000Z</SealedTime>
  <ETag>"e"</ETag>
  <Type>Appendable</Type>
  <Size>1</Size>
  <StorageClass>Standard</StorageClass>
</Contents>
</ListBucketResult>"#;
        let parsed: ListObjectsV1Result = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.contents.len(), 2);
        assert_eq!(parsed.contents[0].transition_time.as_deref(), Some("2024-04-23T07:21:42.000Z"));
        assert_eq!(parsed.contents[0].storage_class, StorageClass::ColdArchive);
        assert_eq!(parsed.contents[1].sealed_time.as_deref(), Some("2020-05-21T12:07:15.000Z"));
        assert_eq!(parsed.contents[1].object_type, ObjectType::Appendable);
    }

    #[test]
    fn parse_response_with_empty_max_keys() {
        // Archive-bucket sample in the OSS docs emits <MaxKeys></MaxKeys>.
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult>
<Name>examplebucket</Name>
<Prefix></Prefix>
<Marker></Marker>
<MaxKeys></MaxKeys>
<Delimiter></Delimiter>
<IsTruncated>false</IsTruncated>
</ListBucketResult>"#;
        let parsed: ListObjectsV1Result = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.max_keys, None);
    }
}
