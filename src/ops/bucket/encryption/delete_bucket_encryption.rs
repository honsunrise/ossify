//! DeleteBucketEncryption.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketencryption>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketEncryptionParams {
    encryption: OnlyKeyField,
}

pub struct DeleteBucketEncryption;

impl Ops for DeleteBucketEncryption {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketEncryptionParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketEncryptionParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketEncryptionParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketEncryptionOps {
    /// Delete the bucket's default server-side encryption rule.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketencryption>
    fn delete_bucket_encryption(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketEncryptionOps for Client {
    async fn delete_bucket_encryption(&self) -> Result<()> {
        self.request(DeleteBucketEncryption).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&DeleteBucketEncryptionParams::default()).unwrap(),
            "encryption"
        );
    }
}
