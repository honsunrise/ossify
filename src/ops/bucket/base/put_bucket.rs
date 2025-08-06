use std::collections::HashMap;
use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::EmptyBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Request};

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

/// Put bucket operation
pub struct PutBucket {
    pub config: PutBucketConfiguration,
    pub options: Option<PutBucketOptions>,
}

impl Ops for PutBucket {
    type Response = EmptyResponseProcessor;
    type Body = EmptyBody;
    type Query = ();

    const PRODUCT: &'static str = "oss";

    fn method(&self) -> Method {
        Method::PUT
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
        let ops = PutBucket { config, options };
        self.request(ops).await
    }
}
