//! PutBucketPolicy.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketpolicy>

use std::future::Future;

use bytes::Bytes;
use http::Method;
use serde::Serialize;

use crate::body::BytesBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketPolicyParams {
    policy: OnlyKeyField,
}

/// The `PutBucketPolicy` operation.
pub struct PutBucketPolicy {
    /// Raw JSON policy document.
    pub policy: Bytes,
}

impl Ops for PutBucketPolicy {
    type Response = EmptyResponseProcessor;
    type Body = BytesBody;
    type Query = PutBucketPolicyParams;

    fn prepare(self) -> Result<Prepared<PutBucketPolicyParams, (Bytes, &'static str)>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketPolicyParams::default()),
            body: Some((self.policy, "application/json")),
            ..Default::default()
        })
    }
}

pub trait PutBucketPolicyOps {
    /// Set the bucket authorization policy (JSON).
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketpolicy>
    fn put_bucket_policy(&self, policy: impl Into<Bytes>) -> impl Future<Output = Result<()>>;
}

impl PutBucketPolicyOps for Client {
    async fn put_bucket_policy(&self, policy: impl Into<Bytes>) -> Result<()> {
        self.request(PutBucketPolicy {
            policy: policy.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&PutBucketPolicyParams::default()).unwrap(), "policy");
    }

    #[test]
    fn prepared_uses_put_with_json_body() {
        let json = br#"{"Version":"1","Statement":[]}"#;
        let prepared = PutBucketPolicy {
            policy: Bytes::from_static(json),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::PUT);
        let (bytes, ct) = prepared.body.unwrap();
        assert_eq!(ct, "application/json");
        assert_eq!(bytes.len(), json.len());
    }
}
