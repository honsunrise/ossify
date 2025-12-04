use std::collections::HashMap;
use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Prepared, Request, ser};

/// Represents the access control list (ACL) for a bucket in Aliyun OSS.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BucketAcl {
    #[serde(rename = "public-read-write")]
    PublicReadWrite,
    #[serde(rename = "public-read")]
    PublicRead,
    #[default]
    #[serde(rename = "private")]
    Private,
}

impl BucketAcl {
    pub fn as_str(&self) -> &str {
        match self {
            BucketAcl::PublicReadWrite => "public-read-write",
            BucketAcl::PublicRead => "public-read",
            BucketAcl::Private => "private",
        }
    }
}

impl AsRef<str> for BucketAcl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Storage class types
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StorageClass {
    #[default]
    #[serde(rename = "Standard")]
    Standard,
    #[serde(rename = "IA")]
    InfrequentAccess,
    #[serde(rename = "Archive")]
    Archive,
    #[serde(rename = "ColdArchive")]
    ColdArchive,
    #[serde(rename = "DeepColdArchive")]
    DeepColdArchive,
}

impl StorageClass {
    pub fn as_str(&self) -> &str {
        match self {
            StorageClass::Standard => "Standard",
            StorageClass::InfrequentAccess => "IA",
            StorageClass::Archive => "Archive",
            StorageClass::ColdArchive => "ColdArchive",
            StorageClass::DeepColdArchive => "DeepColdArchive",
        }
    }
}

/// Data redundancy type
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DataRedundancyType {
    #[default]
    #[serde(rename = "LRS")]
    LocallyRedundantStorage,
    #[serde(rename = "ZRS")]
    ZoneRedundantStorage,
}

impl DataRedundancyType {
    pub fn as_str(&self) -> &str {
        match self {
            DataRedundancyType::LocallyRedundantStorage => "LRS",
            DataRedundancyType::ZoneRedundantStorage => "ZRS",
        }
    }
}

/// Configuration for creating a bucket
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutBucketConfiguration {
    pub storage_class: Option<String>,
    pub data_redundancy_type: Option<String>,
}

/// Options for creating a bucket
#[derive(Debug, Clone, Default)]
pub struct PutBucketOptions {
    pub acl: Option<BucketAcl>,
    pub resource_group_id: Option<String>,
    pub tags: HashMap<String, String>,
}

impl PutBucketOptions {
    fn into_headers(self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        if let Some(acl) = self.acl {
            headers.insert(HeaderName::from_static("x-oss-acl"), acl.as_str().parse()?);
        }
        if let Some(resource_group_id) = self.resource_group_id {
            headers.insert(HeaderName::from_static("x-oss-resource-group-id"), resource_group_id.parse()?);
        }
        if !self.tags.is_empty() {
            let tags_str = ser::to_string(&self.tags)?;
            headers.insert(HeaderName::from_static("x-oss-bucket-tagging"), tags_str.parse()?);
        }

        Ok(headers)
    }
}

/// Put bucket operation
pub struct PutBucket {
    pub config: PutBucketConfiguration,
    pub options: PutBucketOptions,
}

impl Ops for PutBucket {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<PutBucketConfiguration>;
    type Query = ();

    fn prepare(self) -> Result<Prepared<(), PutBucketConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            body: Some(self.config),
            headers: Some(self.options.into_headers()?),
            ..Default::default()
        })
    }
}

pub trait PutBucketOps {
    /// Create a new bucket
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucket>
    fn put_bucket(
        &self,
        config: PutBucketConfiguration,
        options: Option<PutBucketOptions>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutBucketOps for Client {
    async fn put_bucket(
        &self,
        config: PutBucketConfiguration,
        options: Option<PutBucketOptions>,
    ) -> Result<()> {
        let ops = PutBucket {
            config,
            options: options.unwrap_or_default(),
        };
        self.request(ops).await
    }
}
