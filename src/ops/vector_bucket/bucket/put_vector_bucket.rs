//! PutVectorBucket: create a new vector bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putvectorbucket>

use std::future::Future;

use http::Method;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// The `PutVectorBucket` operation. Creates a new vector bucket on the
/// dedicated `oss-vectors` endpoint.
pub struct PutVectorBucket;

impl Ops for PutVectorBucket {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = ();

    fn prepare(self) -> Result<Prepared<()>> {
        Ok(Prepared {
            method: Method::PUT,
            ..Default::default()
        })
    }
}

pub trait PutVectorBucketOps {
    /// Create a new vector bucket. The bucket name is taken from the
    /// `Client` configuration; each account is capped at 10 vector buckets
    /// per region by default.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putvectorbucket>
    fn put_vector_bucket(&self) -> impl Future<Output = Result<()>>;
}

impl PutVectorBucketOps for Client {
    async fn put_vector_bucket(&self) -> Result<()> {
        self.request(PutVectorBucket).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_method() {
        let prepared = PutVectorBucket.prepare().unwrap();
        assert_eq!(prepared.method, Method::PUT);
        assert!(prepared.query.is_none());
        assert!(prepared.headers.is_none());
    }
}
