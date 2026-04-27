//! PutAccessPointPublicAccessBlock.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putaccesspointpublicaccessblock>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::PublicAccessBlockConfiguration;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct PutAccessPointPublicAccessBlockParams {
    #[serde(rename = "publicAccessBlock")]
    public_access_block: OnlyKeyField,
    #[serde(rename = "x-oss-access-point-name")]
    pub access_point_name: String,
}

impl PutAccessPointPublicAccessBlockParams {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            public_access_block: OnlyKeyField,
            access_point_name: name.into(),
        }
    }
}

pub struct PutAccessPointPublicAccessBlock {
    pub name: String,
    pub block_public_access: bool,
}

impl Ops for PutAccessPointPublicAccessBlock {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<PublicAccessBlockConfiguration>;
    type Query = PutAccessPointPublicAccessBlockParams;

    fn prepare(
        self,
    ) -> Result<Prepared<PutAccessPointPublicAccessBlockParams, PublicAccessBlockConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutAccessPointPublicAccessBlockParams::new(self.name)),
            body: Some(PublicAccessBlockConfiguration {
                block_public_access: self.block_public_access,
            }),
            ..Default::default()
        })
    }
}

pub trait PutAccessPointPublicAccessBlockOps {
    /// Enable or disable Block Public Access for an access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putaccesspointpublicaccessblock>
    fn put_access_point_public_access_block(
        &self,
        name: impl Into<String>,
        block_public_access: bool,
    ) -> impl Future<Output = Result<()>>;
}

impl PutAccessPointPublicAccessBlockOps for Client {
    async fn put_access_point_public_access_block(
        &self,
        name: impl Into<String>,
        block_public_access: bool,
    ) -> Result<()> {
        self.request(PutAccessPointPublicAccessBlock {
            name: name.into(),
            block_public_access,
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&PutAccessPointPublicAccessBlockParams::new("ap-01")).unwrap();
        assert_eq!(q, "publicAccessBlock&x-oss-access-point-name=ap-01");
    }
}
