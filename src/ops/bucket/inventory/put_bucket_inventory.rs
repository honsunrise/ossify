//! PutBucketInventory.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketinventory>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::InventoryConfiguration;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct PutBucketInventoryParams {
    inventory: OnlyKeyField,
    /// Must equal the `Id` in the request body.
    #[serde(rename = "inventoryId")]
    pub inventory_id: String,
}

impl PutBucketInventoryParams {
    pub fn new(inventory_id: impl Into<String>) -> Self {
        Self {
            inventory: OnlyKeyField,
            inventory_id: inventory_id.into(),
        }
    }
}

pub struct PutBucketInventory {
    pub config: InventoryConfiguration,
}

impl Ops for PutBucketInventory {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<InventoryConfiguration>;
    type Query = PutBucketInventoryParams;

    fn prepare(self) -> Result<Prepared<PutBucketInventoryParams, InventoryConfiguration>> {
        let id = self.config.id.clone();
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketInventoryParams::new(id)),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketInventoryOps {
    /// Configure an inventory rule for the bucket. The rule's `id` is used as
    /// both the `inventoryId` query parameter and the `<Id>` body element, so
    /// it must be set in `config`.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketinventory>
    fn put_bucket_inventory(&self, config: InventoryConfiguration) -> impl Future<Output = Result<()>>;
}

impl PutBucketInventoryOps for Client {
    async fn put_bucket_inventory(&self, config: InventoryConfiguration) -> Result<()> {
        self.request(PutBucketInventory { config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::common::{
        IncludedObjectVersions,
        InventoryDestination,
        InventoryFormat,
        InventoryFrequency,
        InventorySchedule,
        OssBucketDestination,
    };

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&PutBucketInventoryParams::new("r1")).unwrap();
        assert_eq!(q, "inventory&inventoryId=r1");
    }

    #[test]
    fn prepared_matches_inventory_id() {
        let cfg = InventoryConfiguration {
            id: "report1".to_string(),
            is_enabled: true,
            destination: InventoryDestination {
                oss_bucket_destination: OssBucketDestination {
                    format: InventoryFormat::Csv,
                    account_id: "0".to_string(),
                    role_arn: "r".to_string(),
                    bucket: "b".to_string(),
                    ..Default::default()
                },
            },
            schedule: InventorySchedule {
                frequency: InventoryFrequency::Daily,
            },
            included_object_versions: IncludedObjectVersions::Current,
            ..Default::default()
        };
        let prepared = PutBucketInventory { config: cfg }.prepare().unwrap();
        assert_eq!(prepared.method, Method::PUT);
        assert_eq!(prepared.query.as_ref().unwrap().inventory_id, "report1");
    }
}
