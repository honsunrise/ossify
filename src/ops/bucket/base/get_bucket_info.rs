use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::Owner;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Get bucket info request parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketInfoParams {
    #[serde(rename = "bucketInfo")]
    bucket_info: OnlyKeyField,
}

/// Bucket detail information
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketDetail {
    pub name: String,
    pub location: String,
    pub creation_date: String,
    pub extranet_endpoint: String,
    pub intranet_endpoint: String,
    pub region: String,
    pub storage_class: String,
    pub data_redundancy_type: String,
    pub access_monitor: String,
    pub block_public_access: bool,
    pub transfer_acceleration: String,
    pub cross_region_replication: String,
    pub resource_group_id: Option<String>,
    pub comment: Option<String>,
    pub versioning: Option<String>,
    pub owner: Owner,
}

/// Get bucket info operation
pub struct GetBucketInfo {}

impl Ops for GetBucketInfo {
    type Response = BodyResponseProcessor<BucketDetail>;
    type Body = NoneBody;
    type Query = GetBucketInfoParams;

    fn prepare(self) -> Result<Prepared<GetBucketInfoParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketInfoParams {
                bucket_info: OnlyKeyField,
            }),
            ..Default::default()
        })
    }
}

pub trait GetBucketInfoOps {
    /// Get bucket information
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketinfo>
    fn get_bucket_info(&self) -> impl Future<Output = Result<BucketDetail>>;
}

impl GetBucketInfoOps for Client {
    async fn get_bucket_info(&self) -> Result<BucketDetail> {
        let ops = GetBucketInfo {};
        self.request(ops).await
    }
}
