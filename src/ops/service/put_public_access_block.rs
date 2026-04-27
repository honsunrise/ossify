//! PutPublicAccessBlock — globally enable/disable Block Public Access for OSS.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putpublicaccessblock>

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
pub struct PutPublicAccessBlockParams {
    #[serde(rename = "publicAccessBlock")]
    public_access_block: OnlyKeyField,
}

pub struct PutPublicAccessBlock {
    pub block_public_access: bool,
}

impl Ops for PutPublicAccessBlock {
    const USE_BUCKET: bool = false;

    type Response = EmptyResponseProcessor;
    type Body = XMLBody<PublicAccessBlockConfiguration>;
    type Query = PutPublicAccessBlockParams;

    fn prepare(self) -> Result<Prepared<PutPublicAccessBlockParams, PublicAccessBlockConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutPublicAccessBlockParams::default()),
            body: Some(PublicAccessBlockConfiguration {
                block_public_access: self.block_public_access,
            }),
            ..Default::default()
        })
    }
}

pub trait PutPublicAccessBlockOps {
    /// Globally enable/disable Block Public Access for OSS resources.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putpublicaccessblock>
    fn put_public_access_block(&self, block_public_access: bool) -> impl Future<Output = Result<()>>;
}

impl PutPublicAccessBlockOps for Client {
    async fn put_public_access_block(&self, block_public_access: bool) -> Result<()> {
        self.request(PutPublicAccessBlock { block_public_access }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutPublicAccessBlockParams::default()).unwrap(),
            "publicAccessBlock"
        );
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<PutPublicAccessBlock as Ops>::USE_BUCKET);
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
