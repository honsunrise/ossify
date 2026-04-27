//! GetBucketPolicy.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketpolicy>

use std::future::Future;

use bytes::Bytes;
use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BinaryResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketPolicyParams {
    policy: OnlyKeyField,
}

pub struct GetBucketPolicy;

impl Ops for GetBucketPolicy {
    type Response = BinaryResponseProcessor;
    type Body = NoneBody;
    type Query = GetBucketPolicyParams;

    fn prepare(self) -> Result<Prepared<GetBucketPolicyParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketPolicyParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketPolicyOps {
    /// Retrieve the raw JSON policy document configured on the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketpolicy>
    fn get_bucket_policy(&self) -> impl Future<Output = Result<Bytes>>;
}

impl GetBucketPolicyOps for Client {
    async fn get_bucket_policy(&self) -> Result<Bytes> {
        self.request(GetBucketPolicy).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetBucketPolicyParams::default()).unwrap(), "policy");
    }

    #[test]
    fn prepared_uses_get() {
        let prepared = GetBucketPolicy.prepare().unwrap();
        assert_eq!(prepared.method, Method::GET);
    }
}
