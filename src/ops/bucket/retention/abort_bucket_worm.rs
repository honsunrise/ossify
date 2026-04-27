//! AbortBucketWorm: delete an unlocked retention policy.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/abortbucketworm>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct AbortBucketWormParams {
    worm: OnlyKeyField,
}

/// The `AbortBucketWorm` operation.
pub struct AbortBucketWorm;

impl Ops for AbortBucketWorm {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = AbortBucketWormParams;

    fn prepare(self) -> Result<Prepared<AbortBucketWormParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(AbortBucketWormParams::default()),
            ..Default::default()
        })
    }
}

pub trait AbortBucketWormOps {
    /// Delete the bucket's retention policy. Only possible before the policy
    /// is locked (i.e. while it is in the `InProgress` state and within the
    /// 24-hour window after creation).
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/abortbucketworm>
    fn abort_bucket_worm(&self) -> impl Future<Output = Result<()>>;
}

impl AbortBucketWormOps for Client {
    async fn abort_bucket_worm(&self) -> Result<()> {
        self.request(AbortBucketWorm).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_adds_worm_subresource() {
        let q = crate::ser::to_string(&AbortBucketWormParams::default()).unwrap();
        assert_eq!(q, "worm");
    }

    #[test]
    fn prepared_uses_delete() {
        let prepared = AbortBucketWorm.prepare().unwrap();
        assert_eq!(prepared.method, Method::DELETE);
        assert!(prepared.body.is_none());
    }
}
