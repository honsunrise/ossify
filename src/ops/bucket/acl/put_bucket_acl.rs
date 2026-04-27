//! PutBucketAcl: configure or modify the ACL of a bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketacl>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::ZeroBody;
use crate::error::Result;
use crate::ops::common::BucketAcl;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`PutBucketAcl`] (only the `?acl` sub-resource).
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketAclParams {
    acl: OnlyKeyField,
}

/// The `PutBucketAcl` operation.
pub struct PutBucketAcl {
    pub acl: BucketAcl,
}

impl Ops for PutBucketAcl {
    type Response = EmptyResponseProcessor;
    type Body = ZeroBody;
    type Query = PutBucketAclParams;

    fn prepare(self) -> Result<Prepared<PutBucketAclParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-acl"), self.acl.as_str().parse()?);

        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketAclParams::default()),
            headers: Some(headers),
            body: Some(()),
            ..Default::default()
        })
    }
}

pub trait PutBucketAclOps {
    /// Configure or modify the ACL of the bucket. Overwrites any previous
    /// ACL. The ACL is supplied via the `x-oss-acl` request header, not in
    /// the request body.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketacl>
    fn put_bucket_acl(&self, acl: BucketAcl) -> impl Future<Output = Result<()>>;
}

impl PutBucketAclOps for Client {
    async fn put_bucket_acl(&self, acl: BucketAcl) -> Result<()> {
        self.request(PutBucketAcl { acl }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_adds_acl_subresource() {
        let q = crate::ser::to_string(&PutBucketAclParams::default()).unwrap();
        assert_eq!(q, "acl");
    }

    #[test]
    fn prepared_sets_header_and_method() {
        let prepared = PutBucketAcl {
            acl: BucketAcl::PublicRead,
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::PUT);
        let header = prepared
            .headers
            .as_ref()
            .unwrap()
            .get("x-oss-acl")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(header, "public-read");
    }

    #[test]
    fn prepared_sets_private_acl() {
        let prepared = PutBucketAcl {
            acl: BucketAcl::Private,
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.headers.as_ref().unwrap().get("x-oss-acl").unwrap(), "private");
    }
}
