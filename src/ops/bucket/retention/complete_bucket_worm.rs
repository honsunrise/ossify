//! CompleteBucketWorm: lock an `InProgress` retention policy.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/completebucketworm>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::ZeroBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`CompleteBucketWorm`].
#[derive(Debug, Clone, Serialize)]
pub struct CompleteBucketWormParams {
    #[serde(rename = "wormId")]
    worm_id: String,
}

impl CompleteBucketWormParams {
    pub fn new(worm_id: impl Into<String>) -> Self {
        Self {
            worm_id: worm_id.into(),
        }
    }
}

/// The `CompleteBucketWorm` operation.
pub struct CompleteBucketWorm {
    pub worm_id: String,
}

impl Ops for CompleteBucketWorm {
    type Response = EmptyResponseProcessor;
    type Body = ZeroBody;
    type Query = CompleteBucketWormParams;

    fn prepare(self) -> Result<Prepared<CompleteBucketWormParams>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(CompleteBucketWormParams::new(self.worm_id)),
            body: Some(()),
            ..Default::default()
        })
    }
}

pub trait CompleteBucketWormOps {
    /// Lock a retention policy so the retention period can no longer be
    /// shortened or the policy deleted.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/completebucketworm>
    fn complete_bucket_worm(&self, worm_id: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl CompleteBucketWormOps for Client {
    async fn complete_bucket_worm(&self, worm_id: impl Into<String>) -> Result<()> {
        let ops = CompleteBucketWorm {
            worm_id: worm_id.into(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_includes_worm_id() {
        let q = crate::ser::to_string(&CompleteBucketWormParams::new("1666E2CFB2B3418****")).unwrap();
        assert_eq!(q, "wormId=1666E2CFB2B3418%2A%2A%2A%2A");
    }

    #[test]
    fn prepared_uses_post() {
        let prepared = CompleteBucketWorm {
            worm_id: "abc".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::POST);
        assert_eq!(crate::ser::to_string(prepared.query.as_ref().unwrap()).unwrap(), "wormId=abc");
    }
}
