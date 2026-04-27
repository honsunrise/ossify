//! GetBucketInventory.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketinventory>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::InventoryConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetBucketInventoryParams {
    inventory: OnlyKeyField,
    #[serde(rename = "inventoryId")]
    pub inventory_id: String,
}

impl GetBucketInventoryParams {
    pub fn new(inventory_id: impl Into<String>) -> Self {
        Self {
            inventory: OnlyKeyField,
            inventory_id: inventory_id.into(),
        }
    }
}

pub struct GetBucketInventory {
    pub inventory_id: String,
}

impl Ops for GetBucketInventory {
    type Response = BodyResponseProcessor<InventoryConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketInventoryParams;

    fn prepare(self) -> Result<Prepared<GetBucketInventoryParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketInventoryParams::new(self.inventory_id)),
            ..Default::default()
        })
    }
}

pub trait GetBucketInventoryOps {
    /// Retrieve a single inventory rule by ID.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketinventory>
    fn get_bucket_inventory(
        &self,
        inventory_id: impl Into<String>,
    ) -> impl Future<Output = Result<InventoryConfiguration>>;
}

impl GetBucketInventoryOps for Client {
    async fn get_bucket_inventory(&self, inventory_id: impl Into<String>) -> Result<InventoryConfiguration> {
        self.request(GetBucketInventory {
            inventory_id: inventory_id.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&GetBucketInventoryParams::new("list1")).unwrap();
        assert_eq!(q, "inventory&inventoryId=list1");
    }
}
