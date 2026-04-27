//! DeleteBucketDataAccelerator.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketdataaccelerator>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteBucketDataAcceleratorParams {
    #[serde(rename = "dataAccelerator")]
    data_accelerator: OnlyKeyField,
    #[serde(rename = "x-oss-datalake-cache-available-zone")]
    pub available_zone: String,
}

impl DeleteBucketDataAcceleratorParams {
    pub fn new(available_zone: impl Into<String>) -> Self {
        Self {
            data_accelerator: OnlyKeyField,
            available_zone: available_zone.into(),
        }
    }
}

pub struct DeleteBucketDataAccelerator {
    pub available_zone: String,
}

impl Ops for DeleteBucketDataAccelerator {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketDataAcceleratorParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketDataAcceleratorParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketDataAcceleratorParams::new(self.available_zone)),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketDataAcceleratorOps {
    /// Delete the OSS accelerator configuration for a specific zone.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketdataaccelerator>
    fn delete_bucket_data_accelerator(
        &self,
        available_zone: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketDataAcceleratorOps for Client {
    async fn delete_bucket_data_accelerator(&self, available_zone: impl Into<String>) -> Result<()> {
        self.request(DeleteBucketDataAccelerator {
            available_zone: available_zone.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&DeleteBucketDataAcceleratorParams::new("cn-wulanchabu-b")).unwrap();
        assert_eq!(q, "dataAccelerator&x-oss-datalake-cache-available-zone=cn-wulanchabu-b");
    }

    #[test]
    fn method_is_delete() {
        let p = DeleteBucketDataAccelerator {
            available_zone: "cn-wulanchabu-b".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::DELETE);
    }
}
