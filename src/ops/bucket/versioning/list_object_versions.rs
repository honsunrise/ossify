//! ListObjectVersions (GetBucketVersions).
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listobjectversions>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::{EncodingType, Owner, StorageClass};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`ListObjectVersions`].
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListObjectVersionsParams {
    versions: OnlyKeyField,
    pub delimiter: Option<String>,
    pub key_marker: Option<String>,
    pub version_id_marker: Option<String>,
    pub max_keys: Option<u32>,
    pub prefix: Option<String>,
    pub encoding_type: Option<EncodingType>,
}

impl ListObjectVersionsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }

    pub fn key_marker(mut self, marker: impl Into<String>) -> Self {
        self.key_marker = Some(marker.into());
        self
    }

    pub fn version_id_marker(mut self, marker: impl Into<String>) -> Self {
        self.version_id_marker = Some(marker.into());
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

/// A non-delete-marker object version.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    pub key: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
    pub is_latest: bool,
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub etag: Option<String>,
    #[serde(rename = "Type")]
    pub object_type: Option<String>,
    pub size: Option<u64>,
    pub storage_class: Option<StorageClass>,
    pub owner: Option<Owner>,
    pub transition_time: Option<String>,
    pub restore_info: Option<String>,
}

/// A delete marker entry in the versions listing.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMarker {
    pub key: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
    pub is_latest: bool,
    pub last_modified: String,
    pub owner: Option<Owner>,
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

/// Response body (XML root `<ListVersionsResult>`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectVersionsResult {
    pub name: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub key_marker: String,
    #[serde(default)]
    pub version_id_marker: String,
    pub next_key_marker: Option<String>,
    pub next_version_id_marker: Option<String>,
    pub max_keys: u32,
    #[serde(default)]
    pub delimiter: String,
    pub encoding_type: Option<EncodingType>,
    pub is_truncated: bool,
    #[serde(default, rename = "Version")]
    pub versions: Vec<ObjectVersion>,
    #[serde(default, rename = "DeleteMarker")]
    pub delete_markers: Vec<DeleteMarker>,
    #[serde(default, deserialize_with = "unwrap_common_prefixes")]
    pub common_prefixes: Vec<String>,
}

pub struct ListObjectVersions {
    pub params: ListObjectVersionsParams,
}

impl Ops for ListObjectVersions {
    type Response = BodyResponseProcessor<ListObjectVersionsResult>;
    type Body = NoneBody;
    type Query = ListObjectVersionsParams;

    fn prepare(self) -> Result<Prepared<ListObjectVersionsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListObjectVersionsOps {
    /// List all object versions in a bucket, including delete markers.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listobjectversions>
    fn list_object_versions(
        &self,
        params: Option<ListObjectVersionsParams>,
    ) -> impl Future<Output = Result<ListObjectVersionsResult>>;
}

impl ListObjectVersionsOps for Client {
    async fn list_object_versions(
        &self,
        params: Option<ListObjectVersionsParams>,
    ) -> Result<ListObjectVersionsResult> {
        self.request(ListObjectVersions {
            params: params.unwrap_or_default(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_default_serialize() {
        let q = crate::ser::to_string(&ListObjectVersionsParams::new()).unwrap();
        assert_eq!(q, "versions");
    }

    #[test]
    fn params_full_serialize() {
        let q = crate::ser::to_string(
            &ListObjectVersionsParams::new()
                .delimiter("/")
                .key_marker("example")
                .version_id_marker("CAEQ")
                .max_keys(100)
                .prefix("fun")
                .encoding_type(EncodingType::Url),
        )
        .unwrap();
        assert_eq!(
            q,
            "delimiter=%2F&encoding-type=url&key-marker=example&max-keys=100&prefix=fun&version-id-marker=CAEQ&versions"
        );
    }

    #[test]
    fn parse_single_version() {
        let xml = r#"<ListVersionsResult>
  <Name>bucket</Name>
  <Prefix/>
  <KeyMarker/>
  <VersionIdMarker/>
  <MaxKeys>1000</MaxKeys>
  <IsTruncated>false</IsTruncated>
  <Version>
    <Key>example-object-1.jpg</Key>
    <VersionId/>
    <IsLatest>true</IsLatest>
    <LastModified>2019-08-5T12:03:10.000Z</LastModified>
    <ETag>5B3C1A2E053D763E1B669CC607C5A0FE1****</ETag>
    <Size>20</Size>
    <StorageClass>Standard</StorageClass>
    <Owner><ID>1</ID><DisplayName>u</DisplayName></Owner>
    <TransitionTime>2024-04-23T07:21:42.000Z</TransitionTime>
  </Version>
</ListVersionsResult>"#;
        let parsed: ListObjectVersionsResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.versions.len(), 1);
        assert_eq!(parsed.versions[0].key, "example-object-1.jpg");
        assert_eq!(parsed.versions[0].size, Some(20));
        assert_eq!(parsed.versions[0].storage_class, Some(StorageClass::Standard));
    }

    #[test]
    fn parse_version_and_delete_marker() {
        let xml = r#"<ListVersionsResult>
  <Name>bucket</Name>
  <KeyMarker>example</KeyMarker>
  <VersionIdMarker>CAEQMx</VersionIdMarker>
  <MaxKeys>100</MaxKeys>
  <IsTruncated>false</IsTruncated>
  <DeleteMarker>
    <Key>example</Key>
    <VersionId>CAEQ2</VersionId>
    <IsLatest>false</IsLatest>
    <LastModified>2019-04-09T07:27:28.000Z</LastModified>
    <Owner><ID>1</ID><DisplayName>u</DisplayName></Owner>
  </DeleteMarker>
  <Version>
    <Key>example</Key>
    <VersionId>CAEQ3</VersionId>
    <IsLatest>false</IsLatest>
    <LastModified>2019-04-09T07:27:28.000Z</LastModified>
    <ETag>"250F"</ETag>
    <Type>Normal</Type>
    <Size>93731</Size>
    <StorageClass>Standard</StorageClass>
    <Owner><ID>1</ID><DisplayName>u</DisplayName></Owner>
  </Version>
</ListVersionsResult>"#;
        let parsed: ListObjectVersionsResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.delete_markers.len(), 1);
        assert_eq!(parsed.versions.len(), 1);
        assert_eq!(parsed.delete_markers[0].version_id, "CAEQ2");
    }
}
