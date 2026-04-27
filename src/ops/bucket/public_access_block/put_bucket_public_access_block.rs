//! PutBucketPublicAccessBlock.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketpublicaccessblock>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::PublicAccessBlockConfiguration;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketPublicAccessBlockParams {
    #[serde(rename = "publicAccessBlock")]
    public_access_block: OnlyKeyField,
}

pub struct PutBucketPublicAccessBlock {
    pub block_public_access: bool,
}

impl Ops for PutBucketPublicAccessBlock {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<PublicAccessBlockConfiguration>;
    type Query = PutBucketPublicAccessBlockParams;

    fn prepare(self) -> Result<Prepared<PutBucketPublicAccessBlockParams, PublicAccessBlockConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketPublicAccessBlockParams::default()),
            body: Some(PublicAccessBlockConfiguration {
                block_public_access: self.block_public_access,
            }),
            ..Default::default()
        })
    }
}

pub trait PutBucketPublicAccessBlockOps {
    /// Enable or disable Block Public Access on the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketpublicaccessblock>
    fn put_bucket_public_access_block(&self, block_public_access: bool) -> impl Future<Output = Result<()>>;
}

impl PutBucketPublicAccessBlockOps for Client {
    async fn put_bucket_public_access_block(&self, block_public_access: bool) -> Result<()> {
        self.request(PutBucketPublicAccessBlock { block_public_access })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutBucketPublicAccessBlockParams::default()).unwrap(),
            "publicAccessBlock"
        );
    }

    #[test]
    fn body_serializes() {
        let cfg = PublicAccessBlockConfiguration {
            block_public_access: true,
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<BlockPublicAccess>true</BlockPublicAccess>"));
    }
}
