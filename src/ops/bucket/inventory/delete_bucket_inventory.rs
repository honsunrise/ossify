//! DeleteBucketInventory.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketinventory>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteBucketInventoryParams {
    inventory: OnlyKeyField,
    #[serde(rename = "inventoryId")]
    pub inventory_id: String,
}

impl DeleteBucketInventoryParams {
    pub fn new(inventory_id: impl Into<String>) -> Self {
        Self {
            inventory: OnlyKeyField,
            inventory_id: inventory_id.into(),
        }
    }
}

pub struct DeleteBucketInventory {
    pub inventory_id: String,
}

impl Ops for DeleteBucketInventory {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketInventoryParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketInventoryParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketInventoryParams::new(self.inventory_id)),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketInventoryOps {
    /// Delete an inventory rule by ID.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketinventory>
    fn delete_bucket_inventory(&self, inventory_id: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketInventoryOps for Client {
    async fn delete_bucket_inventory(&self, inventory_id: impl Into<String>) -> Result<()> {
        self.request(DeleteBucketInventory {
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
        let q = crate::ser::to_string(&DeleteBucketInventoryParams::new("list1")).unwrap();
        assert_eq!(q, "inventory&inventoryId=list1");
    }
}
