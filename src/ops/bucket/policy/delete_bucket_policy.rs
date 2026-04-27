//! DeleteBucketPolicy.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketpolicy>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketPolicyParams {
    policy: OnlyKeyField,
}

pub struct DeleteBucketPolicy;

impl Ops for DeleteBucketPolicy {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteBucketPolicyParams;

    fn prepare(self) -> Result<Prepared<DeleteBucketPolicyParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteBucketPolicyParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeleteBucketPolicyOps {
    /// Delete the bucket's authorization policy.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebucketpolicy>
    fn delete_bucket_policy(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketPolicyOps for Client {
    async fn delete_bucket_policy(&self) -> Result<()> {
        self.request(DeleteBucketPolicy).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&DeleteBucketPolicyParams::default()).unwrap(), "policy");
    }

    #[test]
    fn prepared_uses_delete() {
        let prepared = DeleteBucketPolicy.prepare().unwrap();
        assert_eq!(prepared.method, Method::DELETE);
    }
}
